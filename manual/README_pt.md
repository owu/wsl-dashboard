# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Um painel de gestão de instâncias WSL (Windows Subsystem for Linux) moderno, de alto desempenho e leve. Construído com Rust e Slint para uma experiência nativa de alta qualidade.

---

```diff
Aviso:

- Este software não é distribuído através do Microsoft Store.
- Qualquer aplicativo listado lá sob o mesmo nome não é autorizado e pode ser falsificado.
- Por favor, não o baixe para evitar possíveis fraudes.
```

---

<p align="left">
  <a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/rust-v1.92+-orange.svg" alt="Rust" /></a>
  <a href="https://slint.dev" target="_blank"><img src="https://img.shields.io/badge/UI-Slint-blue.svg" alt="Slint" /></a>
  <a href="https://tokio.rs" target="_blank"><img src="https://img.shields.io/badge/Async-Tokio-000000.svg" alt="Tokio" /></a>
  <a href="https://github.com/microsoft/windows-rs" target="_blank"><img src="https://img.shields.io/badge/OS-Windows-0078D6.svg" alt="Windows" /></a>
  <a href="../LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue.svg" alt="License" /></a>
  <a href="https://hellogithub.com/repository/owu/wsl-dashboard" target="_blank"><img src="https://api.hellogithub.com/v1/widgets/recommend.svg?rid=cb1edc45846e475da1dae615a4b4f71c&claim_uid=mWIRuYqZo1FUrjE&theme=small" alt="Featured｜HelloGitHub" /></a>
</p>

I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | Português | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md)

---

## 📑 Índice
- [🌍 Idiomas Suportados](#-idiomas-suportados)
- [🚀 Funcionalidades Principais e Utilização](#-funcionalidades-principais-e-utilização)
- [⚙️ Configuração e Registos](#️-configuração-e-registos)
- [🖼️ Capturas de Ecrã](#️-capturas-de-ecrã)
- [🎬 Demonstração de Funcionamento](#-demonstração-de-funcionamento)
- [💻 Requisitos do Sistema](#-requisitos-do-sistema)
- [📦 Guia de Instalação](#-guia-de-instalação)
- [🛠️ Tecnologias e Desempenho](#️-tecnologias-e-desempenho)
- [⭐️ Trabalho de amor](#️-trabalho-de-amor)
- [📄 Licença](#-licença)

---

## 🌍 Idiomas Suportados

Inglês, Chinês, Chinês, Hindi, Espanhol, Francês, Arabic, Bengali, Português, Russo, Urdu, Indonésio, Alemão, Japonês, Turco, Korean, Italiano, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic

<p align="left">
  <img src="../assets/flags/us.svg" width="32" title="Inglês" alt="Inglês" />
  <img src="../assets/flags/cn.svg" width="32" title="Chinês (Simplificado)" alt="Chinês (Simplificado)" />
  <img src="../assets/flags/tw.svg" width="32" title="Chinês (Tradicional)" alt="Chinês (Tradicional)" />
  <img src="../assets/flags/in.svg" width="32" title="Hindi" alt="Hindi" />
  <img src="../assets/flags/es.svg" width="32" title="Espanhol" alt="Espanhol" />
  <img src="../assets/flags/fr.svg" width="32" title="Francés" alt="Francés" />
  <img src="../assets/flags/sa.svg" width="32" title="Árabe" alt="Árabe" />
  <img src="../assets/flags/bd.svg" width="32" title="Bengali" alt="Bengali" />
  <img src="../assets/flags/pt.svg" width="32" title="Português" alt="Português" />
  <img src="../assets/flags/ru.svg" width="32" title="Russo" alt="Russo" />
  <img src="../assets/flags/pk.svg" width="32" title="Urdu" alt="Urdu" />
  <img src="../assets/flags/id.svg" width="32" title="Indonésio" alt="Indonésio" />
  <img src="../assets/flags/de.svg" width="32" title="Alemão" alt="Alemão" />
  <img src="../assets/flags/jp.svg" width="32" title="Japonesa" alt="Japonesa" />
  <img src="../assets/flags/tr.svg" width="32" title="Turco" alt="Turco" />
  <img src="../assets/flags/kr.svg" width="32" title="Coreano" alt="Coreano" />
  <img src="../assets/flags/it.svg" width="32" title="Italiano" alt="Italiano" />
  <img src="../assets/flags/nl.svg" width="32" title="Holandês" alt="Holandês" />
  <img src="../assets/flags/se.svg" width="32" title="Sueco" alt="Sueco" />
  <img src="../assets/flags/cz.svg" width="32" title="Checo" alt="Checo" />
  <img src="../assets/flags/gr.svg" width="32" title="Grego" alt="Grego" />
  <img src="../assets/flags/hu.svg" width="32" title="Húngaro" alt="Húngaro" />
  <img src="../assets/flags/il.svg" width="32" title="Hebraico" alt="Hebraico" />
  <img src="../assets/flags/no.svg" width="32" title="Norueguês" alt="Norueguês" />
  <img src="../assets/flags/dk.svg" width="32" title="Dinamarquês" alt="Dinamarquês" />
  <img src="../assets/flags/fi.svg" width="32" title="Finlandês" alt="Finlandês" />
  <img src="../assets/flags/sk.svg" width="32" title="Eslovaco" alt="Eslovaco" />
  <img src="../assets/flags/si.svg" width="32" title="Esloveno" alt="Esloveno" />
  <img src="../assets/flags/is.svg" width="32" title="Islandês" alt="Islandês" />
</p>


## 🚀 Funcionalidades Principais e Utilização

- **Interface Nativa Moderna**: GUI intuitiva com suporte para modo escuro/claro, animações suaves e renderização de alto desempenho via **Skia**.
- **Integração com a Área de Notificação**: Suporte total para minimizar para a bandeja (~10MB de RAM), duplo clique para alternar e um menu de contexto funcional.
- **Arranque Inteligente**: Configuração do painel para iniciar com o Windows, minimizar para a bandeja (modo silencioso com `/silent`) e encerramento automático das distribuições ao sair.
- **Controlo Completo de Instâncias**: Iniciar, parar, terminar e desregistar num clique. Monitorização do estado em tempo real e informações detalhadas sobre utilização de disco e localização de ficheiros.
- **Gestão de Distros**: Definir como predefinida, migração (mover o VHDX para outras unidades) e exportação/clonagem para formatos `.tar` ou `.tar.gz`.
- **Integração Rápida**: Lançamento instantâneo do Terminal, VS Code ou Explorador de Ficheiros com diretórios de trabalho personalizáveis e ganchos de script de arranque.
- **Instalação Inteligente**: Instalação a partir da Microsoft Store, GitHub ou ficheiros locais (RootFS/VHDX). Inclui assistente de transferência de RootFS integrado.
- **Segurança Global**: Bloqueios mutex para operações seguras de migração/backup concorrentes e limpeza automática de Appx ao remover.
- **Pegada de Memória Ultra Baixa**: Altamente otimizado para eficiência. O arranque silencioso (tray) utiliza apenas **~10MB** de RAM. O uso em modo janela varia conforme a complexidade do tipo de letra: **~18MB** para idiomas padrão e **~38MB** para idiomas com grandes conjuntos de caracteres (Chinês, Japonês, Coreano).
- **Redes avançadas**: Gerenciamento contínuo de encaminhamento de portas (com criação automática de regras de firewall) e configuração global de proxy HTTP para conectividade unificada.
- **Gerenciamento de Dispositivos USB**: Integração total com `usbipd-win` para uma vinculação, anexação e gerenciamento fáceis de dispositivos USB locais em suas instâncias do WSL, diretamente da interface do painel.


## ⚙️ Configuração e Registos

Toda a configuração é gerida através da vista Definições:

- Escolha o diretório de instalação padrão para as novas instâncias WSL.
- Configure o diretório de registos e o nível de registo (Error / Warn / Info / Debug / Trace).
- Escolha o idioma da interface ou deixe-o seguir o idioma do sistema.
- Alterne o modo escuro e se a aplicação pode encerrar automaticamente o WSL após operações.
- Configure a frequência com que a aplicação verifica atualizações (diariamente, semanalmente, quinzenalmente, mensalmente).
- Ative o arranque automático no boot do sistema (com reparação automática de caminhos).
- Configure a aplicação para minimizar para a bandeja ao iniciar.
- Configure o botão de fechar para minimizar para a bandeja em vez de sair do programa.
- Personalize a barra lateral alternando a visibilidade de guias de recursos específicos.

Os ficheiros de registo são gravados no diretório configurado e podem ser anexados ao reportar problemas.


## 🖼️ Capturas de Ecrã

### Início (Modos Escuro e Claro)
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

<p align="center">
  <img src="../assets/screenshot/home-settings.png" width="48%" />
  <img src="../assets/screenshot/home-configs.png" width="48%" />
</p>

### USB & Menu recolhido
<p align="center">
  <img src="../assets/screenshot/usb.png" width="48%" />
  <img src="../assets/screenshot/collapsed.png" width="48%" />
</p>

### rede
<p align="center">
  <img src="../assets/screenshot/port-forwarding.png" width="48%" />
  <img src="../assets/screenshot/http-proxy.png" width="48%" />
</p>

### Adicionar Instância & Definições
<p align="center">
  <img src="../assets/screenshot/add.png" width="48%" />
  <img src="../assets/screenshot/settings.png" width="48%" />
</p>

### Sobre
<p align="center">
  <img src="../assets/screenshot/about.png" width="48%" />
</p>

## 🎬 Demonstração de Funcionamento

Abaixo está uma demonstração do WSL Dashboard em ação:

![Demonstração do WSL Dashboard](../assets/screenshot/demo.gif)



## 💻 Requisitos do Sistema

- Windows 10 ou Windows 11 com WSL ativado (recomenda-se WSL 2).
- Pelo menos uma distribuição WSL instalada, ou permissão para instalar novas.
- CPU de 64 bits; recomenda-se 4 GB de RAM ou mais para uma utilização fluida.

## 📦 Guia de Instalação

### Opção 1: Descarregar o binário pré-compilado

A forma mais fácil de começar é utilizar a versão já compilada:

1. Vá para a página de [GitHub Releases](https://github.com/owu/wsl-dashboard/releases).
2. Transfira o executável `wsldashboard` mais recente para Windows.
3. Extraia (si necessário) e execute `wsldashboard.exe`.

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
   - Criar uma compilação de lançamento otimizada através do script:

     > O script de compilação requer o conjunto de ferramentas `x86_64-pc-windows-msvc`.

     ```powershell
     .\build\scripts\build.ps1
     ```


## 🛠️ Tecnologias e Desempenho

- **Núcleo**: Implementado em Rust para segurança de memória e abstrações de custo zero.
- **Framework de UI**: Slint com backend de renderização **Skia** de alto desempenho.
- **Runtime Assíncrono**: Tokio para comandos de sistema e E/S não bloqueantes.
- **Destaques de Desempenho**:
  - **Capacidade de resposta**: Arranque quase instantâneo e monitorização do estado WSL em tempo real.
  - **Eficiência**: Utilização de recursos ultra-baixa (detalhes em [Funcionalidades Principais](#-funcionalidades-principais-e-utilização)).
  - **Portabilidade**: O build otimizado produz um único executável compacto.



## ⭐️ Trabalho de amor

Se achou este projeto útil, ficaria grato se pudesse deixar uma estrela no GitHub. O seu apoio ajuda a chegar a um público mais vasto e é profundamente apreciado. É este incentivo que me motiva a continuar a construir.

## 📄 Licença

Este projeto está licenciado sob a GPL-3.0 – consulte o ficheiro [LICENSE](../LICENSE) para mais detalhes.

---

Built with ❤️ for the WSL Community.

---

## 🤝 Community Support

A big thank you to the following communities for their support:

- [Rust Programming Language](https://www.rust-lang.org) - For the powerful and safe programming language
- [Slint | Declarative GUI for Rust, C++, JavaScript & Python](https://slint.dev) - For the modern UI framework
- [WSL: Windows Subsystem for Linux](https://github.com/microsoft/WSL) - For the amazing Windows Subsystem for Linux
- [Tokio - An asynchronous Rust runtime](https://tokio.rs) - For the efficient async runtime
- [Windows Developer Community](https://developer.microsoft.com/en-us/windows/community) - For continuous platform improvements
- [Reddit](https://www.reddit.com) - For global community discussions and support
- [Linux.do](https://linux.do) - For popular community for IT professionals
- [V2EX](https://www.v2ex.com) - For Chinese tech community discussions

Your contributions and feedback make this project possible!

