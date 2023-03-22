<a id="readme-top"></a>

<br />
<div align="center">
  <h3 align="center">Word finder</h3>

  <p align="center">
    Will help you to find word in directory
    <br />
    <br />
    <a href="https://github.com/Ptyg/FileWordFinder_rust">Code</a>
    |
    <a href="https://github.com/Ptyg/FileWordFinder_rust/pulls">Request Feature</a>
  </p>
</div>

<p align="center">
</p>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#installation">Installation</a>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

## About The Project

Utility that searches for a specific word in all files in directory, or you can just put a path to file - this also works.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Installation

Clone the repository:

```sh
git clone https://github.com/Ptyg/FileWordFinder_rust.git
```

And run the following command in the project directory:

```sh
cargo build --release

# or debug, if you want to
cargo build --debug
```

This will build the program and create an executable in the `target/release` ( `target/debug` ) directory.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

To find word in directory run the following command:

```sh
finder {word} {directory}
```

Replace `{word}` with the word you want to search for and `{directory}` with the directory you want to search in.

For example, to search for the word `"hello"` in the directory `/home/user/Documents`, run the following command:

```sh
finder hello /home/user/Documents
```

To find word in specific file run the same command but write full path to file:

```sh
finder hello /home/user/Documents/myFile.txt
```

`Finder` also supports the following optional arguments:

- `-h` or `--help`: Prints help information
- `-V` or `--version`: Prints version information
- `-t` or `--filetype`: Specifies the type of file in which to search for a word
- `-p` or `--path`: Sets path to file or directory
- `-w` or `--word`: Sets word to find

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b { branch_name }`)
3. Commit your Changes (`git commit -m 'Add an amazing feature'`)
4. Push to the Branch (`git push origin { branch_name }`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>