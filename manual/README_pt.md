# WSL Dashboard

<p align="center">
  <img src="../assets/logo/logo.png" width="128" height="128" alt="WSL Dashboard Logo" />
</p>

Um painel moderno, de alto desempenho, leve e com baixo consumo de memória para gerenciamento de instâncias do WSL (Windows Subsystem for Linux). Criado com Rust e Slint para uma experiência nativa premium.

```diff
Aviso:

- WSL Dashboard não é distribuído através da Microsoft Store.
- Qualquer aplicativo listado lá sob o nome "WSL Dashboard" é não autorizado e pode ser falsificado.
- Por favor, não baixe para evitar possíveis golpes.
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

README I18N :  [English](../README.md) | [简体中文](./README_zh_CN.md) | [繁體中文](./README_zh_TW.md) | [हिन्दी](./README_hi.md) | [Español](./README_es.md) | [Français](./README_fr.md) | [العربية](./README_ar.md) | [বাংলা](./README_bn.md) | Português | [Русский](./README_ru.md) | [اردو](./README_ur.md) | [Bahasa Indonesia](./README_id.md) | [Deutsch](./README_de.md) | [日本語](./README_ja.md) | [Türkçe](./README_tr.md) | [한국어](./README_ko.md) | [Italiano](./README_it.md) | [Nederlands](./README_nl.md) | [Svenska](./README_sv.md) | [Čeština](./README_cs.md) | [Ελληνικά](./README_el.md) | [Magyar](./README_hu.md) | [עברית](./README_he.md) | [Norsk](./README_no.md) | [Dansk](./README_da.md) | [Suomi](./README_fi.md) | [Slovenčina](./README_sk.md) | [Slovenščina](./README_sl.md) | [Íslenska](./README_is.md) | [Tiếng Việt](./README_vi.md) | [తెలుగు](./README_te.md) | [Basa Jawa](./README_jv.md) | [ภาษาไทย](./README_th.md) | [தமிழ்](./README_ta.md) | [Filipino](./README_fil.md) | [ਪੰਜਾਬੀ](./README_pa.md) | [Bahasa Melayu](./README_ms.md) | [Polski](./README_pl.md) | [Українська](./README_uk.md) | [فارسی](./README_fa.md) | [ಕನ್ನಡ](./README_kn.md) | [मराठी](./README_mr.md) | [Hausa](./README_ha.md) | [မြန်မာ](./README_my.md) | [Oʻzbek](./README_uz.md) | [Azərbaycan](./README_az.md) | [Cebuano](./README_ceb.md) | [മലയാളം](./README_ml.md) | [سنڌي](./README_sd.md) | [አማርኛ](./README_am.md)

---

## 📑 Índice
[🌍 Idiomas Suportados](#-idiomas-suportados) | [🚀 Funcionalidades Principais e Utilização](#-funcionalidades-principais-e-utilização) | [🖼️ Capturas de Ecrã](#️-capturas-de-ecrã) | [📦 Guia de Instalação](#-guia-de-instalação) | [📢 Grupos Oficiais da Comunidade](#-grupos-oficiais-da-comunidade) | [❤️ Apoiar este projeto](#️-apoiar-este-projeto) | [⭐️ Trabalho de amor](#️-trabalho-de-amor) | [📄 Licença](#-licença)

---

## 🌍 Idiomas Suportados

Inglês, Chinês, Chinês, Hindi, Espanhol, Francês, Arabic, Bengali, Português, Russo, Urdu, Indonésio, Alemão, Japonês, Turco, Korean, Italiano, Dutch, Swedish, Czech, Greek, Hungarian, Hebrew, Norwegian, Danish, Finnish, Slovak, Slovenian, Icelandic, Vietnamita, Telugu, Javanês, Tailandês, Tâmil, Filipino, Punjabi, Malaio, Polonês, Ucraniano, Persa, Canarês, Marata, Hauçá, Birmanês, Uzbeque, Azeri, Cebuano, Malaiala, Sindhi, Amárico

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
  <img src="../assets/flags/vn.svg" width="32" title="Vietnamita" alt="Vietnamita" />
  <img src="../assets/flags/in.svg" width="32" title="Telugu" alt="Telugu" />
  <img src="../assets/flags/id.svg" width="32" title="Javanês" alt="Javanês" />
  <img src="../assets/flags/th.svg" width="32" title="Tailandês" alt="Tailandês" />
  <img src="../assets/flags/in.svg" width="32" title="Tâmil" alt="Tâmil" />
  <img src="../assets/flags/ph.svg" width="32" title="Filipino" alt="Filipino" />
  <img src="../assets/flags/pk.svg" width="32" title="Punjabi" alt="Punjabi" />
  <img src="../assets/flags/my.svg" width="32" title="Malaio" alt="Malaio" />
  <img src="../assets/flags/pl.svg" width="32" title="Polonês" alt="Polonês" />
  <img src="../assets/flags/ua.svg" width="32" title="Ucraniano" alt="Ucraniano" />
  <img src="../assets/flags/ir.svg" width="32" title="Persa" alt="Persa" />
  <img src="../assets/flags/in.svg" width="32" title="Canarês" alt="Canarês" />
  <img src="../assets/flags/in.svg" width="32" title="Marata" alt="Marata" />
  <img src="../assets/flags/ng.svg" width="32" title="Hauçá" alt="Hauçá" />
  <img src="../assets/flags/mm.svg" width="32" title="Birmanês" alt="Birmanês" />
  <img src="../assets/flags/uz.svg" width="32" title="Uzbeque" alt="Uzbeque" />
  <img src="../assets/flags/az.svg" width="32" title="Azeri" alt="Azeri" />
  <img src="../assets/flags/ph.svg" width="32" title="Cebuano" alt="Cebuano" />
  <img src="../assets/flags/in.svg" width="32" title="Malaiala" alt="Malaiala" />
  <img src="../assets/flags/pk.svg" width="32" title="Sindhi" alt="Sindhi" />
  <img src="../assets/flags/et.svg" width="32" title="Amárico" alt="Amárico" />
</p>

## 🚀 Funcionalidades Principais e Utilização

- **Interface Nativa Moderna**: GUI intuitiva com suporte para modo escuro/claro, animações suaves e renderização de alto desempenho via **Skia**.
- **Integração com a Área de Notificação**: Suporte total para minimizar para a bandeja (~10MB de RAM), duplo clique para alternar e um menu de contexto funcional.
- **Arranque Inteligente**: Configuração do painel para iniciar com o Windows, minimizar para a bandeja (modo silencioso com `/silent`) e encerramento automático das distribuições ao sair.
- **Agendamento de tarefas**: Gestão flexível de agendamento de tarefas, permitindo a execução automática de scripts e comandos em instâncias WSL em horários ou intervalos especificados para operações automatizadas.
- **Controlo Completo de Instâncias**: Iniciar, parar, terminar e desregistar num clique. Monitorização do estado em tempo real e informações detalhadas sobre utilização de disco e localização de ficheiros.
- **Gestão de Distros**: Definir como predefinida, migração (mover o VHDX para outras unidades) e exportação/clonagem para formatos `.tar` ou `.tar.gz`.
- **Montagem de disco**: Monte e gerencie diretamente os discos virtuais VHDX das instâncias WSL, suportando operações de montagem, desmontagem e gestão de disco, facilitando a transferência de ficheiros entre o anfitrião e o WSL.
- **Integração Rápida**: Lançamento instantâneo do Terminal, VS Code ou Explorador de Ficheiros com diretórios de trabalho personalizáveis e ganchos de script de arranque.
- **Instalação de Distribuição**: Instale distribuições Linux através da Microsoft Store, GitHub, ficheiros locais (RootFS/VHDX) ou espelhos online (com teste de velocidade automático para escolher o espelho mais rápido e assistente de transferência de RootFS integrado).
- **Segurança Global**: Bloqueios mutex para operações seguras de migração/backup concorrentes e limpeza automática de Appx ao remover.
- **Pegada de Memória Ultra Baixa**: Altamente otimizado para eficiência. O arranque silencioso (tray) utiliza apenas **~10MB** de RAM. O uso em modo janela varia conforme a complexidade do tipo de letra: **~18MB** para idiomas padrão e **~38MB** para idiomas com grandes conjuntos de caracteres (Chinês, Japonês, Coreano).
- **Redes avançadas**: Gerenciamento contínuo de encaminhamento de portas (com criação automática de regras de firewall) e configuração global de proxy HTTP para conectividade unificada.
- **Gerenciamento de Dispositivos USB**: Integração total com `usbipd-win` para uma vinculação, anexação e gerenciamento fáceis de dispositivos USB locais em suas instâncias do WSL, diretamente da interface do painel.
- **Configuração e logs**: Gerencie nas Configurações o diretório de instalação, o diretório e nível de logs, o idioma, o modo escuro, a frequência de atualização, a inicialização automática e o comportamento ao fechar, além da barra lateral.
- **Manter distros ativas**: Um clique em «Iniciar» mantém a distro em execução em vez de encerrá-la após ~60 s ociosa; ou ative «Iniciar esta distro ao iniciar o Windows».
- **Atalhos globais**: Defina atalhos globais para mostrar/ocultar a janela principal ou abrir o terminal de uma distro.


## 🖼️ Capturas de Ecrã
<p align="center">
  <img src="../assets/screenshot/home.png" width="48%" />
  <img src="../assets/screenshot/home-dark.png" width="48%" />
</p>

Quer ver o painel em ação? Visite o [site oficial](https://www.wslui.com/) para conferir a galeria completa de capturas de tela.

## 📦 Guia de Instalação

### Opção 1: Visitar o site do projeto (Recomendado)

Recomendamos visitar o site oficial para descarregar, pois oferece vários links de espelho para uma experiência mais suave:

Vá para a [página de Download](https://www.wslui.com/download/) e escolha o espelho adequado para sua região.

### Opção 2: Instalar via winget

Pode instalar o WSLDashboard diretamente do Windows Package Manager (winget), usando o moniker ou o identificador completo do pacote:

```powershell
# Search (case-insensitive)
winget search wsl-dashboard
# or
winget search WSLDashboard

# Install (pick one)
winget install wsl-dashboard
# or
winget install Owu.WSLDashboard
```

> O identificador do pacote winget é `Owu.WSLDashboard` e o moniker é `wsl-dashboard` (insensível a maiúsculas/minúsculas). Qualquer um funciona.

Para mais informações, visite o [repositório comunitário do WinGet](https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Owu/WSLDashboard).

### Opção 3: Compilar a partir do código-fonte

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
     .\build\portable\build.ps1 -Arch x64
     ```


## 📢 Grupos Oficiais da Comunidade

Bem-vindo aos grupos oficiais da comunidade WSL UI! Junte-se aos nossos canais para discutir dicas de uso, relatar problemas, obter as últimas atualizações e conectar-se com os desenvolvedores e outros usuários.

<div align="center">

| Grupo Global (Telegram) | Grupo Chinês (WeChat) |
| :---: | :---: |
| <img src="https://cdn1.wslui.com/static/images/group/group-telegram.png" width="240" alt="Telegram Group QR Code" /> | <img src="https://cdn1.wslui.com/static/images/group/group-wechat.png" width="240" alt="WeChat Group QR Code" /> |
| Escaneie para entrar | Escaneie para entrar |

</div>


## ❤️ Apoiar este projeto

- Este projeto está licenciado sob a GPL-3.0 e é gratuito para todos os utilizadores.
- Desde o desenvolvimento de funcionalidades e testes diários até à correção de erros, todo o trabalho é feito no tempo livre. O caminho do código aberto não é fácil de percorrer sozinho — o seu reconhecimento e apoio dão ao projeto a confiança para continuar.
- Se esta ferramenta realmente o ajudou, considere dar uma mão. Todas as doações são destinadas a custos de servidor, iterações de versão e melhorias de funcionalidades, mantendo o projeto continuamente atualizado e em progresso constante.
- Cada pequeno ato de bondade é um raio de luz estelar. Obrigado novamente pela sua compreensão e generosidade！ [Visite nossa página de doações](https://www.wslui.com/donate/).


## ⭐️ Trabalho de amor

Se achou este projeto útil, ficaria grato se pudesse deixar uma estrela no GitHub. O seu apoio ajuda a chegar a um público mais vasto e é profundamente apreciado. É este incentivo que me motiva a continuar a construir.


## 📄 Licença

Este projeto está licenciado sob a GPL-3.0 – consulte o ficheiro [LICENSE](../LICENSE) para mais detalhes.
