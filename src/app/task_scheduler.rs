// SPDX-FileCopyrightText: Copyright (c) 2026 owu <wqh@live.com>
// SPDX-License-Identifier: GPL-3.0-only

// Unified timer task scheduler
//
// All scheduled business logic should implement the `ScheduledTask` trait
// and register with `TaskScheduler` for unified execution management.

use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn, error, info};
use crate::AppWindow;

// Task execution interval enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TaskInterval {
    // 5-second interval
    FiveSeconds,
    // 30-second interval
    ThirtySeconds,
    // 5-minute interval
    FiveMinutes,
    // Custom interval (seconds)
    Custom(u64),
}

impl TaskInterval {
    // Convert to Duration
    pub fn to_duration(&self) -> Duration {
        match self {
            TaskInterval::FiveSeconds => Duration::from_secs(5),
            TaskInterval::ThirtySeconds => Duration::from_secs(30),
            TaskInterval::FiveMinutes => Duration::from_secs(300),
            TaskInterval::Custom(secs) => Duration::from_secs(*secs),
        }
    }
}

// Scheduled task trait
#[async_trait::async_trait]
pub trait ScheduledTask: Send + Sync + 'static {
    // Task name, used for log identification
    fn name(&self) -> &str;

    // Task execution interval
    fn interval(&self) -> TaskInterval;

    // Whether window visibility is required to execute (default true)
    fn requires_window_visible(&self) -> bool {
        true
    }

    // Whether this is a one-time task (default false).
    // One-time tasks execute once and stop the interval group.
    // WARNING: One-time tasks may block the group for extended periods (e.g., network polling).
    // Do NOT add high-frequency recurring tasks to a group that contains one-time tasks.
    fn is_oneshot(&self) -> bool {
        false
    }

    // Execute the task
    async fn execute(&self, app_handle: &slint::Weak<AppWindow>) -> Result<(), String>;
}

// Timer task scheduler
pub struct TaskScheduler {
    tasks: Vec<Box<dyn ScheduledTask>>,
    app_handle: slint::Weak<AppWindow>,
}

impl TaskScheduler {
    // Create a new task scheduler
    pub fn new(app_handle: slint::Weak<AppWindow>) -> Self {
        Self {
            tasks: Vec::new(),
            app_handle,
        }
    }

    // Register a scheduled task
    pub fn register<T: ScheduledTask>(&mut self, task: T) {
        info!("scheduler: registering task '{}' with interval {:?}", task.name(), task.interval());
        self.tasks.push(Box::new(task));
    }

    // Start the scheduler, creating an independent timer for each interval group
    pub fn start(self) {
        let tasks = self.tasks;
        let app_handle = self.app_handle;

        if tasks.is_empty() {
            warn!("scheduler: no tasks registered");
            return;
        }

        info!("scheduler: starting with {} tasks", tasks.len());

        // Group tasks by interval
        let mut groups: std::collections::HashMap<u64, Vec<Arc<dyn ScheduledTask>>> = std::collections::HashMap::new();
        for task in tasks {
            let interval_secs = task.interval().to_duration().as_secs();
            groups.entry(interval_secs).or_default().push(Arc::from(task));
        }

        // Create a timer for each interval group
        for (interval_secs, group_tasks) in groups {
            let ah = app_handle.clone();
            let task_count = group_tasks.len();
            let tasks_arc = Arc::new(group_tasks);

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

                info!("scheduler: started {}s interval with {} tasks", interval_secs, task_count);

                // First tick triggers immediately
                loop {
                    interval.tick().await;

                    // Execute all tasks in this interval group concurrently
                    let mut handles = Vec::new();
                    for task in tasks_arc.iter() {
                        let ah = ah.clone();
                        let task = task.clone();
                        let handle = tokio::spawn(async move {
                            let task_name = task.name();
                            let requires_visible = task.requires_window_visible();

                            // If task requires window visibility, check first
                            if requires_visible {
                                let is_visible = {
                                    let ah2 = ah.clone();
                                    let (tx, rx) = std::sync::mpsc::channel();
                                    let _ = slint::invoke_from_event_loop(move || {
                                        let visible = if let Some(app) = ah2.upgrade() {
                                            app.get_is_window_visible()
                                        } else {
                                            false
                                        };
                                        let _ = tx.send(visible);
                                    });
                                    // Wait for closure to complete on Slint event loop thread
                                    rx.recv_timeout(std::time::Duration::from_millis(100)).unwrap_or(false)
                                };

                                if !is_visible {
                                    debug!("scheduler: task '{}' skipped (window not visible)", task_name);
                                    return;
                                }
                            }

                            // Execute the task
                            match task.execute(&ah).await {
                                Ok(()) => debug!("scheduler: task '{}' executed successfully", task_name),
                                Err(e) => error!("scheduler: task '{}' failed: {}", task_name, e),
                            }
                        });
                        handles.push(handle);
                    }

                    // Wait for all tasks to complete
                    for handle in handles {
                        let _ = handle.await;
                    }

                    // oneshot tasks: stop this interval group after first execution
                    if tasks_arc.iter().all(|t| t.is_oneshot()) {
                        info!("scheduler: {}s oneshot group completed, stopping", interval_secs);
                        break;
                    }
                }
            });
        }
    }
}
