# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" />
</p>

Um painel de gestão de instâncias WSL (Windows Subsystem for Linux) moderno, de alto desempenho e leve. Construído com Rust e Slint para uma experiência nativa de alta qualidade.

---

[![Rust](https://img.shields.io/badge/rust-v1.92+-orange.svg)](https://www.rust-lang.org)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

I18N: [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [English](../README.md) | [日本語](./README_ja.md) | [Français](./README_fr.md) | [Español](./README_es.md) | [Русский](./README_ru.md) | Português | [Deutsch](./README_de.md) | [Italiano](./README_it.md) | [Türkçe](./README_tr.md) | [Bahasa Indonesia](./README_id.md) | [हिन्दी](./README_hi.md) | [বাংলা](./README_bn.md)

---

## 🖼️ Capturas de Ecrã

### Início (Modos Claro e Escuro)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

### Adicionar Instância e Definições
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Sobre & Menu recolhido
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

## 🎬 Demonstração de Funcionamento

Abaixo está uma demonstração do WSL Dashboard em ação:

![Demonstração do WSL Dashboard](../assets/screenshot/demo.gif)

## 🚀 Principais Funcionalidades

- Interface gráfica intuitiva com suporte para modo escuro e animações suaves.
- Gestão com um clique de todas as suas distribuições WSL (Iniciar, Parar, Terminar, Desregistar).
- Acesso rápido aos terminais das distribuições, VS Code e Explorador de Ficheiros.
- Monitorização e visualização do estado das instâncias WSL em tempo real.
- Exportação e cópia de segurança para ficheiros `.tar` ou arquivos comprimidos `.tar.gz`.
- Importação e clonagem de instâncias a partir de cópias de segurança ou distribuições existentes.
- Reinstalação de instâncias WSL volumosas (migração de VHDX) para outros discos para poupar espaço na unidade C:.
- Instalação inteligente de distribuições a partir da Microsoft Store ou GitHub.
- Assistente de transferência de RootFS integrado para instalações manuais.
- Informações detalhadas sobre a localização do ficheiro VHDX, tamanho do disco virtual e utilização real do disco.

## Requisitos do Sistema

- Windows 10 ou Windows 11 com WSL ativado (recomenda-se WSL 2).
- Pelo menos uma distribuição WSL instalada, ou permissão para instalar novas.
- CPU de 64 bits; recomenda-se 4 GB de RAM ou mais para uma utilização fluida de várias distribuições.

## 📦 Instalação

### Opção 1: Descarregar o binário pré-compilado

A forma mais fácil de começar é utilizar a versão já compilada:

1. Vá para a página de [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Transfira o executável `wsldashboard` mais recente para Windows.
3. Extraia (se necessário) e execute `wsldashboard.exe`.

Não é necessário instalador; a aplicação é um binário portátil único.

### Opção 2: Compilar a partir do código-fonte

Certifique-se de que tem o conjunto de ferramentas Rust instalado (Rust 1.92+ ou superior).

1. Clone o repositório:

   ```powershell
   git clone https://github.com/owu/wsl-dashboard.git
   cd wsl-dashboard
   ```

2. Compile e execute:

   - Para desenvolvimento:

     ```powershell
     cargo run
     ```

   - Compilação de lançamento otimizada:

     ```powershell
     cargo run --release
     ```

   - Utilizar o script de compilação (recomendado para binários de lançamento):

     > O script de compilação requer o conjunto de ferramentas `x86_64-pc-windows-gnu`.

     ```powershell
     .\build\scripts\build.ps1
     ```

## 🧭 Visão Geral de Utilização

- **Gerir distribuições existentes**: inicie, pare, termine ou desregiste qualquer distribuição WSL a partir da vista principal.
- **Abrir ferramentas rapidamente**: inicie uma distribuição no seu terminal, VS Code ou Explorador de Ficheiros com um único clique.
- **Criar novas instâncias**: utilize a vista Adicionar Instância para instalar a partir da Microsoft Store, transferir imagens RootFS ou clonar distribuições existentes.
- **Cópia de segurança e restauro**: exporte distribuições para arquivos `.tar` / `.tar.gz` e importe-as mais tarde ou noutra máquina.
- **Mover local de armazenamento**: coloque ficheiros VHDX grandes noutra unidade para poupar espaço no disco C:.
- **Monitorizar estado**: acompanhe em tempo real o estado da distribuição e a utilização do armazenamento enquanto o WSL Dashboard está a ser executado.

## ⚙️ Configuração e Registos

Toda a configuração é gerida através da vista Definições:

- Escolha o diretório de instalação predefinido para novas instâncias WSL.
- Configure o diretório de registos e o nível de registo (Error / Warn / Info / Debug / Trace).
- Escolha o idioma da interface ou deixe-o seguir o idioma do sistema.
- Ative ou desative o modo escuro e se a aplicação pode encerrar automaticamente o WSL após as operações.
- Configure a frequência com que a aplicação verifica se existem atualizações (diariamente, semanalmente, quinzenalmente, mensalmente).

Os ficheiros de registo são gravados no diretório configurado e podem ser anexados ao reportar problemas.

## 🛠️ Tecnologias e Desempenho

- **Núcleo**: implementado em Rust para segurança de memória e abstrações de custo zero.
- **Framework de interface de utilizador**: Slint, um kit de ferramentas de interface moderno acelerado por GPU (backend: `winit`).
- **Runtime assíncrono**: Tokio para comandos de sistema e E/S altamente concorrentes e sem bloqueios.
- **Desempenho**:
  - **Utilização de memória**: normalmente cerca de 60–80 MB de RAM.
  - **Capacidade de resposta**: arranque quase instantâneo e atualizações de estado do WSL em tempo real utilizando streaming.
  - **Tamanho do binário**: a compilação de lançamento otimizada produz um único executável compacto.

## 🌍 Idiomas Suportados

É fornecido suporte completo de internacionalização para os seguintes idiomas:

| Idioma | Código | Emoji |
| :--- | :---: | :---: |
| Chinês Simplificado | `zh-CN` | 🇨🇳 |
| Chinês Tradicional | `zh-TW` | 🇭🇰 / 🇹🇼 |
| Inglês | `en` | 🇺🇸 |
| Japonês | `ja` | 🇯🇵 |
| Francês | `fr` | 🇫🇷 |
| Espanhol | `es` | 🇪🇸 |
| Russo | `ru` | 🇷🇺 |
| Português | `pt` | 🇵🇹 |
| Alemão | `de` | 🇩🇪 |
| Italiano | `it` | 🇮🇹 |
| Turco | `tr` | 🇹🇷 |
| Indonésio | `id` | 🇮🇩 |
| Hindi | `hi` | 🇮🇳 |
| Bengali | `bn` | 🇧🇩 |

## 📄 Licença

Este projeto está licenciado sob a GPL-3.0 – consulte o ficheiro [LICENSE](LICENSE) para mais detalhes.

---

Built with ❤️ for the WSL Community.
