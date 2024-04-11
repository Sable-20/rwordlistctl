<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Sable-20/rwordlistctl?style=for-the-badge&logoColor=white)
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GPLv3 License][license-shield]][license-url]
<!-- [![LinkedIn][linkedin-shield]][linkedin-url] -->



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Sable-20/rwordlistctl">
    <img src="assets/images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">RWordlistctl</h3>

  <p align="center">
    Rust wordlistctl! 
    <br />
    <a href="https://github.com/Sable-20/rwordlistctl"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <!-- <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a> --> 
    ·
    <a href="https://github.com/Sable-20/rwordlistctl/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/Sable-20/rwordlistctl/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li>
        <a href="#commands">Commands</a>
        <ul>
            <li><a href="#fetch">Fetch</a></li> 
            <li><a href="#search">Search</a></li> 
            <li><a href="#list">List</a></li> 
        </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li>
        <a href="#contributing">Contributing</a>
        <ul>
            <li><a href="#for-developers">For Developers</a></li>
        </ul>
    </li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

A script to fetch, install, and search wordlist archives from websites offering wordlists with more than 6400 available wordlists!

This is a Rust rewrite of the original tool: `wordlistctl`. The original tool was built in Python and suffered from the same pitfalls as many Python programs:
* Memory intensive
* Slow
* Hard to read and debug
* Excessively verbose

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

[![Rust][Rust.com]][Rust-url]
[![NixOS][Nix.com]][Nix-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

How to install and use this project

### Prerequisites

* Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

#### Building from scratch (locally)

1. Clone the repo
   ```sh
   git clone https://github.com/Sable-20/rwordlistctl.git
   ```
2. Move into directory
   ```sh
   cd rwordlistctl
   ```
3. Install the app
   ```sh
   cargo install . --root /usr/local
   ``` 
4. Copy config file to correct location
   ```sh
   touch ~/.config/rwordlistctl/config.toml && cp config/config.toml ~/.config/rwordlistctl/config.toml
   ```
5. Copy repository file to correct location
   ```sh
   touch ~/.config/rwordlistctl/repo.toml && cp config/repo.toml ~/.config/rwordlsitctl/repo.toml
   ```
#### Building from scratch (from web)
 1. Install from web
    ```sh
    cargo install --git https://github.com/Sable-20/rwordlistctl.git --root /usr/local
    ```
 2. Install config files
    ```sh
    touch ~/.config/rwordlistctl/repo.toml && touch ~/.config/rwordlistctl/config.toml && curl -o ~/.config/rwordlistctl/repo.toml https://raw.githubusercontent.com/Sable-20/rwordlistctl/master/config/repo.toml && curl -o ~/.config/rwordlistctl/config.toml https://raw.githubusercontent.com/Sable-20/rwordlistctl/master/config/config.toml
    ```

#### With docker
Simply run:
```sh
docker build -t <image-name> .
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

#### Locally
Once the binary is built, ensure that it is in your path by running `rwordlistctl` or `wordlistctl` (if you have aliased the command or have installed via `pacman` where it is aliased automatically).

Once you know that the command is working you can run:
```sh
rwordlistctl [OPTIONS] [COMMAND]
```

The help is as follows:
```sh
                                        --==[ rwordlistctl by Blackarch Linux ]==--
Rust rewrite of wordlistctl: Fetch, install and search wordlist archives from websites.

Usage: rwordlistctl [OPTIONS] [COMMAND]

Commands:
  fetch
  search
  list
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to the configuration file [default: /usr/share/rwordlistctl/.config/config.toml]
  -h, --help             Print help
  -V, --version          Print version
```

#### With Docker
> [!CAUTION] 
> DEPRECATION WARNING: Docker support is a non-goal and will not recieve attention, Dockerfiles will be removed in version `1.0.0` to be replaced by `nix` generated Docker images

If you wish to load into a fresh shell within the docker container where the wordlists will be sandboxed and not be loaded onto your host system you may run:
```sh
docker run --rm -ti <image-name>
```

If you wish to connect the container to your host machine so that wordlists will be downloaded to your machine you may run:
```sh
docker run --rm -it --mount "type=bind,src=/usr/share/wordlists,target=/usr/share/wordlists" <image-name>
```
This will make it so that any wordlists downloaded in the container will also be downloaded into `/usr/share/wordlists`!

From here refer back to `running locally`


_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Commands -->
## Commands
Below is the help for various commands and explanations for each/

### Fetch
```sh
                                    --==[ rwordlistctl by Blackarch Linux ]==--
Usage: rwordlistctl.exe fetch [OPTIONS] --wordlist=<WORDLISTS>...

Options:
  -d, --decompress               Decompress and remove the archive file
  -w, --workers=<COUNT>          Number of download workers [default: 10]
  -u, --user-agent=<USER_AGENT>  User agent to use for fetching [default: rwordlistctl/0.1.0]
  -b, --base-dir=<BASE_DIR>      Base directory to store wordlists [default: /usr/share/wordlists]
  -l, --wordlist=<WORDLISTS>...  Wordlist to fetch
  -g, --group=<GROUP>...         Group of wordlists to fetch [possible values: usernames, passwords, discovery, fuzzing, misc]
  -r, --regex                    Use regex to find wordlists with your search term contained within the name
  -h, --help                     Print help
```

### Search

### List

<!-- ROADMAP -->
## Roadmap

- [ ] Use mangen to generate man pages
- [ ] Determine if `list` and `search` are redundant and if we should remove
- [ ] Implement custom config file (TOML format)
- [ ] Write tests
- [ ] Write proper documentation
- [ ] Multi-language Documentation
    - [ ] Chinese
    - [ ] Spanish
    - [ ] French

See the [open issues](https://github.com/Sable-20/rwordlistctl/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- For Developers -->
### For Developers

While developing, in order to reduce errors such as "it worked on my machine" or "it works on my version of Rust" we kindly request that if possible, consider using [`nix`](https://nixos.org). `nix` is a wonderful tool that allows for reproducible development and build environments where we can lock versions in easily. While Docker is an amazing tool and we will continue to support using `Dockerfile` for builds and development until version `1.0.0` we will transition to using `nix flakes` as they are generally more secure. Docker images will still be available however they will be generated by `nix`'s builtin tooling system for generated Docker images that are particularly compact.

Thank you for understanding that supporting Docker fully is a non-goal and will recieve minimal attention during this stage of development.


<!-- LICENSE -->
## License

Distributed under the GPLv3 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/Sable-20/rwordlistctl](https://github.com/Sable-20/rwordlistctl)

[![Issues][issues-shield]][issues-url] Contact current maintainers here

[![X/Twitter][Twitter.com]][Twitter-url] [@blackarch](https://twitter.com/blackarchlinux)
[![E-Mail][Email.com]][Email-url] team[@]blackarch[.]org

[![Website][Website.com]][Website-url] [Blackarch Linux](https://www.blackarch.org/)

[![Matrix][Matrix.com]][Matrix-url] [Matrix](https://matrix.to/#/#BlackArch:matrix.org)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

* Major contributors
  * Sable-20
  * sepehrdaddev
  * Blackarch contributors

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Sable-20/rwordlistctl.svg?style=for-the-badge
[contributors-url]: https://github.com/Sable-20/rwordlistctl/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Sable-20/rwordlistctl.svg?style=for-the-badge
[forks-url]: https://github.com/Sable-20/rwordlistctl/network/members
[stars-shield]: https://img.shields.io/github/stars/Sable-20/rwordlistctl.svg?style=for-the-badge
[stars-url]: https://github.com/Sable-20/rwordlistctl/stargazers
[issues-shield]: https://img.shields.io/github/issues/Sable-20/rwordlistctl.svg?style=for-the-badge
[issues-url]: https://github.com/Sable-20/rwordlistctl/issues
[license-shield]: https://img.shields.io/github/license/Sable-20/rwordlistctl.svg?style=for-the-badge
[license-url]: https://github.com/Sable-20/rwordlistctl/blob/master/LICENSE.txt
<!-- [linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/othneildrew -->
<!-- [product-screenshot]: images/screenshot.png -->
<!-- [Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com -->
[Rust.com]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Nix.com]: https://img.shields.io/badge/NixOS-5277C3?style=for-the-badge&logo=nixos&logoColor=white
[Nix-url]: https://nixos.org
[Matrix.com]: https://img.shields.io/badge/matrix-000000?style=for-the-badge&logo=Matrix&logoColor=white
[Matrix-url]: https://matrix.to
[Website.com]: https://img.shields.io/badge/website-000000?style=for-the-badge&logo=About.me&logoColor=white
[Website-url]: https://www.blackarch.org
[Email.com]: https://img.shields.io/badge/Gmail-D14836?style=for-the-badge&logo=gmail&logoColor=white
[Email-url]: team@blackarch.org
[Twitter.com]: https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white
[Twitter-url]: https://twitter.com/blackarchlinux

-----------------------

## Description

Rust rewrite of original by Blackarch development team. Rewritten in Rust by Sable-20.

Script to fetch, install, update and search wordlist archives from websites
offering wordlists with more than 6400 wordlists available.

In the latest version of the Blackarch Linux it has been added to
**/usr/share/wordlists/** directory.

## Installation

`pacman -S rwordlistctl`


