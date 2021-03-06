use greenhorn::appconfig::AppConfig;
use std::path::PathBuf;

// Functional tests
// They only test the complete rendering of a page
// to ensure consistency after further modifications

#[test]
fn load_config() {
    let _read_conf = AppConfig::new(PathBuf::from("tests/data/Config.toml"));
}

#[tokio::test]
async fn render_page_simple() {
    let expected = "<!DOCTYPE html>
<html>

<head>
    <title>a</title>
    <style>
h1   {color: blue;}
    </style>
    <base href=\"/a/\" />
</head>

<body>
    <header>
        <h1>A heading here</h1>
    </header>
    <article>
<h2>Single page test</h2>
<p>hello from the normal page</p>

    </article>
    <footer>This is a footer</footer>
</body>

</html>";

    let read_conf = AppConfig::new(PathBuf::from("tests/data/Config.toml"));

    let rendered = read_conf.generate_page("a").await.unwrap();

    assert_eq!(rendered, expected);
}

#[tokio::test]
async fn render_homepage() {
    let expected = "<!DOCTYPE html>
<html>

<head>
    <title>a</title>
    <style>
h1   {color: blue;}
    </style>
    <base href=\"/a/\" />
</head>

<body>
    <header>
        <h1>A heading here</h1>
    </header>
    <article>
<h2>Single page test</h2>
<p>hello from the normal page</p>

    </article>
    <footer>This is a footer</footer>
</body>

</html>";

    let read_conf = AppConfig::new(PathBuf::from("tests/data/Config.toml"));

    let rendered = read_conf.generate_homepage().await.unwrap();

    assert_eq!(rendered, expected);
}

#[tokio::test]
async fn render_list() {
    let expected = "<!DOCTYPE html>
<html>

<head>
    <title>b</title>
    <style>
h1   {color: blue;}
    </style>
    <base href=\"/b/\" />
</head>

<body>
    <header>
        <h1>A heading here</h1>
    </header>
    <article>
<ul>
    
    <li>
        <a href=\"l1.md\">l1</a>
    </li>
    
    <li>
        <a href=\"l2.md\">l2</a>
    </li>
    
</ul>
    </article>
    <footer>This is a footer</footer>
</body>

</html>";

    let read_conf = AppConfig::new(PathBuf::from("tests/data/Config.toml"));

    let rendered = read_conf.generate_page("b").await.unwrap();

    assert_eq!(rendered, expected);
}

#[tokio::test]
async fn render_list_page() {
    let expected = "<!DOCTYPE html>
<html>

<head>
    <title>b</title>
    <style>
h1   {color: blue;}
    </style>
    <base href=\"/b/\" />
</head>

<body>
    <header>
        <h1>A heading here</h1>
    </header>
    <article>
<h2>List item 1</h2>

    </article>
    <footer>This is a footer</footer>
</body>

</html>";

    let read_conf = AppConfig::new(PathBuf::from("tests/data/Config.toml"));

    let rendered = read_conf.generate_list_page("b", "l1.md").await.unwrap();

    assert_eq!(rendered, expected);
}
