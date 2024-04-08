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
    <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a> -->
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
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
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

* [![Rust][Rust.com]][Rust-url]

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

#### Building on Docker 
TODO

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

#### Locally
TODO

#### With Docker
TODO


_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Use mangen to generate man pages
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



<!-- LICENSE -->
## License

Distributed under the GPLv3 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@blackarch](https://twitter.com/blackarchlinux) - team[@]blackarch[.]org

Project Link: [https://github.com/Sable-20/rwordlistctl](https://github.com/Sable-20/rwordlistctl)

Website: [Blackarch Linux](https://www.blackarch.org/)

Matrix: [Matrix](https://matrix.to/#/#BlackArch:matrix.org)

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

-----------------------

## Description

Rust rewrite of original by Blackarch development team. Rewritten in Rust by Sable-20.

Script to fetch, install, update and search wordlist archives from websites
offering wordlists with more than 6400 wordlists available.

In the latest version of the Blackarch Linux it has been added to
**/usr/share/wordlists/** directory.

## Installation

`pacman -S rwordlistctl`

## Usage

### With Docker
note: doesnt work rn, fixing with rustls-tls
```sh
docker run --rm -it --mount "type=bind,src=$pwd/testing,target=/testing/usr/share/wordlists" --network host rwordlistctl <COMMANDS>
```

TODO: FIX below, use mangen

TODO: INSERT HELP VIEW HERE

TODO: Write tests

TODO: Convert all `eyre!` calls to handled errors instead of panicking

TODO: implement deflation

TODO: REDO OPTIONS HERE
----------
### Fetch Options
```
$ wordlistctl fetch [-h] [-l WORDLIST [WORDLIST ...]]
                         [-g {usernames,passwords,discovery,fuzzing,misc} [{usernames,passwords,discovery,fuzzing,misc} ...]]
                         [-d] [-w WORKERS] [-u USERAGENT] [-b BASEDIR] fetch_term

positional arguments:
  fetch_term           fetch string filter

optional arguments:
  -h, --help            show this help message and exit

  -l WORDLIST [WORDLIST ...], --wordlist WORDLIST [WORDLIST ...]
                        wordlist to fetch

  -g, --group {group} [{group} ...]
                        wordlist group to fetch
                        available groups:
                          usernames
                          passwords
                          discovery
                          fuzzing
                          misc

  -d, --decompress      decompress and remove archive

  -w WORKERS, --workers WORKERS
                        download workers [default: 10]

  -u USERAGENT, --useragent USERAGENT
                        fetch user agent [default: wordlistctl/v0.9.x]

  -b BASEDIR, --base-dir BASEDIR
                        wordlists base directory [default: /usr/share/wordlists]

```


### Search Options
```
$ wordlistctl search  [-h] [-l] [-b BASEDIR] search_term

positional arguments:
  search_term           what to search

optional arguments:
  -h, --help            show this help message and exit

  -l, --local           search local archives

  -b BASEDIR, --base-dir BASEDIR
                        wordlists base directory [default: /usr/share/wordlists]

  -f INDEX [INDEX ...], --fetch INDEX [INDEX ...]
                        fetch the wordlists at the given indexes in the search results, see
                        fetch options for additional options

fetch options:
  -d, --decompress      decompress and remove archive

  -w WORKERS, --workers WORKERS
                        download workers [default: 10]

  -u USERAGENT, --useragent USERAGENT
                        parser user agent [default: wordlistctl/v0.9.x]
```

### List Options
```
$ wordlistctl list [-h] [-g {usernames,passwords,discovery,fuzzing,misc}]

optional arguments:
  -h, --help            show this help message and exit

  -g, --group {group}
                        show all wordlists in group
                        available groups:
                          usernames
                          passwords
                          discovery
                          fuzzing
                          misc

  -f INDEX [INDEX ...], --fetch INDEX [INDEX ...]
                        fetch the wordlists at the given indexes in the list, see
                        fetch options for additional options

fetch options:
  -d, --decompress      decompress and remove archive

  -w WORKERS, --workers WORKERS
                        download workers [default: 10]

  -u USERAGENT, --useragent USERAGENT
                        parser user agent [default: wordlistctl/v0.9.x]
```

## Get Involved

You can get in touch with the BlackArch Linux team. Just check out the following:

**Please, send us pull requests!**

**Web:** https://www.blackarch.org/

**Mail:** team@blackarch.org

**IRC:** [irc://irc.freenode.net/blackarch](irc://irc.freenode.net/blackarch)
