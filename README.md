# greenhorn

## What it is

Greenhorn is a web application made for personal websites. It uses markdown files to describe the pages and can be a replacement for static site generators.

## How to use

### Configuration
The configuration file follows the TOML syntax.

The required fields are:

* html_template : this combined with the following `css` field describe the header and footer of the page and the overall style what all pages will have
* css
* list_template : this will be the layout of the list pages on the site
* media_dir : directory to put static files (such as images)
* homepage : you must assign a homepage, from the configured list of pages

The pages are also listed in the config file with the following fields:

* name: how the site will refer to the page (also the URL)
* kind: for the moment one of the following: `Normal` or `List`
* source: for the `Normal` page it will be the markdown document, for the `List` page it will be the directory containing markdown documents

An example of a working configuration file:
```toml
# Greenhorn config file

homepage = "a"

html_template = "tests/data/templates/template.html"
css = "tests/data/templates/style.css"
list_template = "tests/data/templates/list.html"
media_dir = "tests/data/media"

[[pages]]
name = "a"
kind = "Single"
source = "tests/data/pages/singletest.md"

[[pages]]
name = "b"
kind = "List"
source = "tests/data/pages/listtest"
```

### Templates

You must create 2 template files (with TinyTemplate syntax) and a CSS file for greenhorn to work. Examples for the 2 are below:

html_template
```html
<!DOCTYPE html>
<html>

<head>
    <title>{title}</title>
    <style>
{css}
    </style>
    <base href="/{title}/" />
</head>

<body>
    <header>
        <h1>A heading here</h1>
    </header>
    <article>
{body}
    </article>
    <footer>This is a footer</footer>
</body>

</html>
```

list_template
```html
<ul>
    {{ for value in list }}
    <li>
        <a href="{value}">{value}</a>
    </li>
    {{ endfor }}
</ul>
```

### Running the application

greenhorn can then be run with a single command

```
greenhorn -c "[config_file]" -a "[address]:[port]"
```