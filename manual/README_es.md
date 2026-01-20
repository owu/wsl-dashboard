# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

Un panel de gestión de instancias de WSL (Windows Subsystem for Linux) moderno, de alto rendimiento y ligero. Construido con Rust y Slint para una experiencia nativa premium.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | Español | [Русский](./README_ru.md) | [Português](./README_pt.md) | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Capturas de Pantalla

### Inicio (Modo Claro y Oscuro)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Añadir Instancia y Configuración
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Acerca de y menú colapsado
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demostración de Funcionamiento

A continuación se muestra una demostración de WSL Dashboard en acción:

![Demostración de WSL Dashboard](../assets/screenshot/demo.gif)

## 🚀 Características Principales

- Interfaz gráfica intuitiva con soporte de modo oscuro y animaciones fluidas.
- Gestión en un clic de todas sus distribuciones WSL (Iniciar, Detener, Terminar, Anular registro).
- Acceso rápido a las terminales de las distribuciones, VS Code y el Explorador de archivos.
- Monitorización y visualización del estado de las instancias de WSL en tiempo real.
- Exportación y copia de seguridad a archivos `.tar` o `.tar.gz` comprimidos.
- Importación y clonación de instancias desde copias de seguridad o distribuciones existentes.
- Reubicación de instancias de WSL grandes (migración VHDX) a otros discos para ahorrar espacio en la unidad C:.
- Instalación inteligente de distribuciones desde Microsoft Store o GitHub.
- Asistente de descarga RootFS integrado para instalaciones manuales.
- Información detallada sobre la ubicación del archivo VHDX, el tamaño del disco virtual y el uso real del disco.

## Requisitos del Sistema

- Windows 10 o Windows 11 con WSL habilitado (se recomienda WSL 2).
- Al menos una distribución de WSL instalada, o permiso para instalar nuevas distribuciones.
- CPU de 64 bits; se recomienda 4 GB de RAM o más para un uso fluido de varias distribuciones.

## 📦 Instalación

### Opción 1: Descargar el binario precompilado

La forma más fácil de empezar es usar el lanzamiento precompilado:

1. Vaya a la página de [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Descargue el último ejecutable `wsldashboard` para Windows.
3. Extraiga (si está empaquetado) y ejecute `wsldashboard.exe`.

No se requiere instalador; la aplicación es un único binario portátil.

### Opción 2: Construir desde el código fuente

Asegúrese de tener instalado el conjunto de herramientas de Rust (Rust 1.92+ o más reciente).

1. Clone el repositorio:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Construir y ejecutar:

   - Para desarrollo:

     ```powershell
     cargo run
     ```

   - Construcción de lanzamiento optimizada:

     ```powershell
     cargo run --release
     ```

   - Uso del script de construcción (recomendado para producir binarios de lanzamiento):

     > El script de construcción requiere el conjunto de herramientas `x86_64-pc-windows-gnu`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Resumen de Uso

- **Gestionar distribuciones existentes**: inicie, detenga, termine o anule el registro de cualquier distribución de WSL desde la vista principal.
- **Abrir herramientas rápidamente**: inicie una distribución en su terminal, VS Code o el Explorador de archivos con un solo clic.
- **Crear nuevas instancias**: use la vista Añadir Instancia para instalar desde Microsoft Store, descargar imágenes RootFS o clonar distribuciones existentes.
- **Copia de seguridad y restauración**: exporte distribuciones a archivos `.tar` / `.tar.gz` e impórtelos más tarde o en otra máquina.
- **Mover la ubicación de almacenamiento**: reubique archivos VHDX grandes a una unidad diferente para ahorrar espacio en la unidad C:.
- **Monitorizar el estado**: vigile el estado de la distribución y el uso del almacenamiento en tiempo real mientras se ejecuta WSL Dashboard.

## ⚙️ Configuración y Registros

Toda la configuración se gestiona a través de la vista Configuración:

- Elija el directorio de instalación por defecto para las nuevas instancias de WSL.
- Configure el directorio de registros y el nivel de registro (Error / Warn / Info / Debug / Trace).
- Seleccione el idioma de la interfaz o deje que siga el idioma del sistema.
- Cambie entre el modo claro y oscuro, y decida si la aplicación puede apagar automáticamente WSL después de las operaciones.
- Configure la frecuencia con la que la aplicación comprueba si hay actualizaciones (diariamente, semanalmente, quincenalmente, mensualmente).

Los archivos de registro se escriben en el directorio configurado y pueden adjuntarse al informar de problemas.

## 🛠️ Tecnologías y Rendimiento

- **Núcleo**: implementado en Rust para la seguridad de la memoria y abstracciones de coste cero.
- **Marco de interfaz de usuario**: Slint, un conjunto de herramientas de interfaz de usuario moderno acelerado por GPU (backend: `winit`).
- **Entorno de ejecución asíncrono**: Tokio para comandos del sistema y entrada/salida altamente concurrentes y no bloqueantes.
- **Rendimiento**:
  - **Uso de memoria**: normalmente alrededor de 60–80 MB de RAM.
  - **Capacidad de respuesta**: arranque casi instantáneo y actualizaciones del estado de WSL en tiempo real mediante streaming.
  - **Tamaño del binario**: la construcción de lanzamiento optimizada produce un único ejecutable compacto.

## 🌍 Idiomas Soportados

Se proporciona soporte completo de internacionalización para los siguientes idiomas:

| Idioma | Código | Emoji |
| :--- | :---: | :---: |
| Chino Simplificado | `zh-CN` | 🇨🇳 |
| Chino Tradicional | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Inglés | `en` | 🇺🇸 |
| Japonés | `ja` | 🇯🇵 |
| Francés | `fr` | 🇫🇷 |
| Español | `es` | 🇪🇸 |
| Ruso | `ru` | 🇷🇺 |
| Portugués | `pt` | 🇵🇹 |
| Alemán | `de` | 🇩🇪 |
| Italiano | `it` | 🇮🇹 |
| Turco | `tr` | 🇹🇷 |
| Indonesio | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengalí | `bn` | 🇧🇩 |

## 📄 Licencia

Este proyecto está bajo la GPL-3.0 – vea el archivo [LICENSE](LICENSE) para más detalles.

---

Built with ❤️ for the WSL Community.
