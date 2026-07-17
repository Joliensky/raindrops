# raindrops 🌧️ 

A rain ripple simulation for your terminal, written in Rust. 


- animation of falling raindrops and expanding water ripples
- customizable options for color, wave size, and frequency




## Installation

### Prerequisites

Since this project is built from source, you will need the [Rust compiler](https://rustup.rs/) installed on your system.

### Linux & macOS

Clone the repository and install it system-wide using `make`:

```sh
git clone https://github.com/Joliensky/raindrops.git
cd raindrops
make
sudo make install
```

This will compile the optimized version and place the executable in `/usr/local/bin/`. 

To uninstall, simply run:
```sh
sudo make uninstall
```

Alternatively, you can install it locally via Cargo:
```sh
cargo install --path .
```

*Note: Make sure your `~/.cargo/bin` directory is added to your system's `PATH` variable so you can run the binary from anywhere.*



## Usage

Simply launch the simulation by typing:
```bash
raindrops
```

### Custom Configuration

You can fully customize the rain simulation using flags. 

| Flag | Long Option | Description | Default |
| :--- | :--- | :--- | :--- |
| `-c` | `--color` | Color of the ripples (`blue`, `green`, `cyan`, `red`, `yellow`, `white`) | `blue` |
| `-s` | `--size` | Maximum size/radius of the expanding waves | `16.0` |
| `-f` | `--frequency` | Probability of new drops spawning per frame (`0.0` to `1.0`) | `0.1` |

### Examples

* **Heavy green rain storm with massive waves:**
  ```bash
  raindrops -c green -s 30 -f 0.4
  ```

* **A gentle, white ambient drizzle:**
  ```bash
  raindrops -c white -s 8 -f 0.03
  ```


**Help Page:**
  ```bash
  raindrops --help
  ```

## Attribution
If you use raindrops, please credit **Joliensky**
(https://github.com/Joliensky/raindrops).

## License

MIT