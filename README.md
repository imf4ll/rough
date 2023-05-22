<div align="center">
    <img src="./assets/showcase.gif" />
    <h3>A GTK based app launcher</h3>
</div>

&nbsp;
## 📜 Big update v0.2.0:

#### Introducing "modules":
In addition of bug fixes, we bring a new system for rough, called "modules", who expands rough uses, adding some new features
and purposes, like the old "calculator" who give fast access to a calculator, and two new modules, "weather" who presents the
current weather on your city and "news" who presents the last news based on your country. We are open to requests and ideas to
news modules, feel free to open a issue.

&nbsp;
## ❗️ Install:

**This program have Linux as target system, we don't give support for any other system**

```bash
git clone https://github.com/imf4ll/rough.git && cd rough
make install
```

&nbsp;

## ❗️ Update:
```bash
git clone https://github.com/imf4ll/rough.git && cd rough
make update
```

&nbsp;

Or, if you maintain the source code
```bash
git pull origin master
make update
```

&nbsp;
## 🚀 Usage:
Running on app-only mode (This is the default mode):
```bash
rough
```

On this mode, you can only run apps, without any shell support

&nbsp;

Running on shell mode:
```bash
rough --shell
```

On this mode, you can run apps and shell commands directly

&nbsp;
<div align="center">

### Available options
| Option | Description |
|--------|-------------|
| -s/--shell | Runs on shell mode |
| -m/--modules | Shows all modules |
| -e/--enable-module | Enables a module (ex: "rough -e calc" or "rough -e 'calc, weather'") |
| -d/--disable-module | Disables a module (ex: "rough -d calc" or "rough -d 'calc, weather'") |

</div>

&nbsp;
## ⚙️ Modules
- Calculator
- Weather (OpenWeather API)
- News (News API)

### Setup weather module
1. Open "config.json" and set "modules" > "weather" > "enable" to true.

2. Browse to <a href="https://home.openweathermap.org/users/sign_up" target="_blank">OpenWeather API</a>.

3. Create an account.

4. Navigate to <a href="https://home.openweathermap.org/api_keys" target="_blank">My API keys</a>.

5. Copy key value. (ex: 1a79a4d60de6718e8e5b326e338ae533)

6. Return to "config.json" and insert API key on "modules" > "weather" > "key".

### Setup news module
1. Open "config.json" and set "modules" > "news" > "enable" to true.

2. Browse to <a href="https://newsapi.org/register" target="_blank">News API</a>.

3. Create an account.

4. Copy your API key.

5. Return to "config.json" and insert API key on "modules" > "news" > "key".

&nbsp;
## ⚙️ Configuration:

Default template: "YOURHOME/.config/rough/config.json"
```json
{
    "window": {
        "width": 600,
        "border_width": 6,
        "border_color": "#252525",
        "opacity": 1.0,
        "background_color": "0, 0, 0",
        "font_color": "#FFFFFF",
        "font": ""
    },
    "container": {
        "max_height": 200
    },
    "textbox": {
        "transparent": false
    },
    "list": {
        "margin_top": 6,
        "transparent": false
    },
    "modules": {
        "calc": true,
        "weather": {
            "enable": false,
            "key": "OpenWeather API key",
            "city": "City name",
            "cache_time": 30, // cache interval in minutes
            "units": "metric" // metric for celsius, imperial for fahrenheit
        },
        "news": {
            "enable": false,
            "key": "News API key",
            "region": "us", // ex: us, br
            "browser": "chrome", // ex: chrome, brave
            "cache_time": 60 // cache interval in minutes
        }
    }
}
```

&nbsp;
## 💅 Available styling options:

<div align="center">

### Window
| Option | Description |
|--------|-------------|
| width | Set window width |
| border_width | Set window border width |
| border_color | Set window border color (like in CSS) |
| opacity | If smaller than 1.0, will set a alpha on the background |
| background_color | If transparent enabled, will be the background color (use values between 0 and 1, like "(0.40, 0.42, 0.54)") |
| font_color | Set font color of entire application |
| font | Set font family (Give only the name, ex: '"font": Inter') |

&nbsp;

### Container (ScrolledWindow)
| Option | Description |
|--------|-------------|
| max_height | Set max height of container |

&nbsp;

### Textbox (Entry)
| Option | Description |
|--------|-------------|
| transparent | If window opacity smaller than 1.0, receive true or false to transparency |

&nbsp;

### List (ListBox)
| Option | Description |
|--------|-------------|
| margin_top | Set margin-top of the list |
| transparent | If window opacity smaller than 1.0, receive true or false to transparency |

</div>
