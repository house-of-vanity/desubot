#[allow(dead_code)]
static CODE_HELP: &str = "<b>Code highlighter</b>

<i>Usage</i>
<pre>/code
&lt;CODE&gt;
#&lt;lang - JS by default&gt; #&lt;theme - Dracula by default&gt;</pre>

Language may be defined by both name and extension - Rust, rs...
Max lines - 80

List of themes:
1337
DarkNeon
Dracula
GitHub
Monokai_Extended
Monokai_Extended_Bright
Monokai_Extended_Light
Monokai_Extended_Origin
Nord
OneHalfDark
OneHalfLight
Solarized_(dark)
Solarized_(light)
Sublime_Snazzy
TwoDark
ansi-dark
ansi-light
base16
base16-256
gruvbox
gruvbox-light
gruvbox-white
zenburn
";

#[allow(dead_code)]
pub static SQL_HELP: &str = "<b>Perform an SQL command</b>
<i>* Only one sentence per message.
* Only SELECT command.
* Max result length is 100 lines. Use LIMIT 100.
* SQLITE syntax is available only.</i>";