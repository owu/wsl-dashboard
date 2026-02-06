// Helper structure for stateful decoding of WSL output streams
#[derive(Default)]
pub(crate) struct WslOutputDecoder {
    pub(crate) is_utf16: Option<bool>,
    pub(crate) buffer: Vec<u8>,
}

impl WslOutputDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode(&mut self, new_bytes: &[u8]) -> String {
        if new_bytes.is_empty() && self.buffer.is_empty() {
            return String::new();
        }
        
        self.buffer.extend_from_slice(new_bytes);

        // Attempt to detect encoding (if not yet determined)
        if self.is_utf16.is_none() {
            // Check BOM
            if self.buffer.len() >= 2 && self.buffer[0] == 0xFF && self.buffer[1] == 0xFE {
                self.is_utf16 = Some(true);
                self.buffer.drain(0..2);
            } else if self.buffer.len() >= 4 {
                // Heuristic detection: count proportion of 0 bytes at even positions (0-indexed 1, 3, 5...)
                let mut null_count = 0;
                let pair_count = self.buffer.len() / 2;
                for i in 0..pair_count {
                    if self.buffer[i * 2 + 1] == 0 {
                        null_count += 1;
                    }
                }
                
                if null_count >= pair_count * 60 / 100 {
                    self.is_utf16 = Some(true);
                } else {
                    // Default fallback to UTF-8
                    self.is_utf16 = Some(false);
                }
            } else {
                // Too little data, cannot determine yet unless already contains 0-byte characteristics
                if self.buffer.iter().any(|&b| b == 0) {
                    // If already saw 0 and length less than 4, might be small packet UTF-16
                    if self.buffer.len() >= 2 && self.buffer[1] == 0 {
                        self.is_utf16 = Some(true);
                    }
                }
                
                // If still not determined, don't decode yet (or fallback if it's simple ASCII without 0)
                if self.buffer.len() < 2 { return String::new(); }
            }
        }

        // Decode according to the determined encoding
        match self.is_utf16 {
            Some(true) => {
                // UTF-16 LE: must be double-byte aligned
                let data_len = self.buffer.len() & !1;
                if data_len == 0 { return String::new(); }
                
                let u16_chars: Vec<u16> = self.buffer[..data_len]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                
                self.buffer.drain(0..data_len);
                String::from_utf16_lossy(&u16_chars)
            }
            Some(false) => {
                self.decode_utf8()
            }
            None => {
                if self.buffer.is_empty() { return String::new(); }
                
                let b0 = self.buffer[0];
                
                // Detect UTF-16 LE: second byte is usually 0 (for ASCII)
                if self.buffer.len() >= 2 && self.buffer[1] == 0 {
                    self.is_utf16 = Some(true);
                    self.decode(&[])
                } 
                // Common ASCII or control characters -> UTF-8
                else if (b0 >= 0x20 && b0 <= 0x7E) || b0 == b'\r' || b0 == b'\n' || b0 == b'\t' {
                    self.is_utf16 = Some(false);
                    self.decode_utf8()
                }
                // BOM detection (UTF-8 or UTF-16)
                else if b0 == 0xEF || b0 == 0xFF || b0 == 0xFE {
                    if self.buffer.len() >= 3 && self.buffer[0] == 0xEF && self.buffer[1] == 0xBB && self.buffer[2] == 0xBF {
                        // UTF-8 BOM
                        self.is_utf16 = Some(false);
                        self.buffer.drain(..3);
                        self.decode(&[])
                    } else if self.buffer.len() >= 2 {
                        if self.buffer[0] == 0xFF && self.buffer[1] == 0xFE {
                            self.is_utf16 = Some(true);
                            self.buffer.drain(..2);
                            self.decode(&[])
                        } else {
                            self.is_utf16 = Some(false);
                            self.decode_utf8()
                        }
                    } else {
                        String::new()
                    }
                }
                // Non-ASCII and non-0 -> likely UTF-8 multi-byte sequence
                else if self.buffer.len() >= 3 {
                    self.is_utf16 = Some(false);
                    self.decode_utf8()
                }
                else {
                    String::new()
                }
            }
        }
    }

    fn decode_utf8(&mut self) -> String {
        // Try to parse as UTF-8. 
        // Note: On Chinese Windows, WSL output might be GBK (CP936) even with WSL_UTF8=1 for some system messages.
        match std::str::from_utf8(&self.buffer) {
            Ok(_) => {
                let s = String::from_utf8_lossy(&self.buffer).to_string();
                self.buffer.clear();
                s
            }
            Err(e) => {
                let valid_len = e.valid_up_to();
                if valid_len > 0 {
                    let s = String::from_utf8_lossy(&self.buffer[..valid_len]).to_string();
                    self.buffer.drain(..valid_len);
                    s
                } else {
                    // If buffer is full of unrecognized characters (> 4 bytes), most likely it's another encoding (like GBK)
                    // We use from_utf8_lossy anyway but we don't clear the buffer unless it's getting too large.
                    // This prevents getting stuck on a single multi-byte sequence.
                    if self.buffer.len() > 8 {
                        let s = String::from_utf8_lossy(&self.buffer).to_string();
                        self.buffer.clear();
                        s
                    } else {
                        String::new()
                    }
                }
            }
        }
    }
}

// Decode byte data to string, automatically detecting UTF-16 LE or UTF-8 encoding
pub fn decode_output(bytes: &[u8]) -> String {
    let mut decoder = WslOutputDecoder::new();
    decoder.decode(bytes)
}
