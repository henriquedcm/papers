# papers

Tool which creates files and calls an user defined text editor.

The purpose of this project is to **quickly create a text file and open it with
a text editor** in an organized structure of directories and files' names.

# Preview

![papers preview](https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExYmM5ZjBsaXlya3hud2lqc2thdDg2MnZremw0ZGltdXR5cnM5dnJ5YiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/dRxBOwlKLosDYcXnJ3/giphy.gif "papers preview")

# Usage

Rust must be installed on the machine. [See also](https://www.rust-lang.org/tools/install).

**1. Clone this repository**

```git clone https://github.com/henriquedcm/papers.git```

**2. Build the release version**

Inside the directory of this repository, build the project:

```cargo build --release```

**3. Copy the binary to the machine**

The build generated a directory `target/release/` and there is a binary
`papers` file inside the directory. Copy this file to `/usr/bin/`.

**4. Set up the environment**

Create the directory `~/.config/papers/` if it doesn't exist. Inside the
directory, create a file `.env` and write the following variables to the file:

- ROOT_DIR
- EDITOR_COMMAND
- EXTENSION

`ROOT_DIR`: the root directory of the notes files. E.g.:
`/home/henrique/notes/`
`EDITOR_COMMAND`: the command which calls the text editor. E.g.: nvim, vi, nano...
`EXTENSION`: extension of the notes files

**5. Create a note**

Firstly, check if the tool is working with:

```papers --version```

Secondly, create a note with a random name such as `testing` in a random topic
such as `installing`:

```papers -t installing -n testing```

In the root directory of the notes a directory `installing/` is created and
a file similar to `20231105022200_testing.md` is created inside the directory.
The numbers before the name represent the system's datetime when the file was
created.

Finally, create a note without a name in a random nested topic such as
`installing.nested`:

```papers -t installing.nested```

In the root directory of the notes a directory `installing/nested/` is created
and a file similar to `2023110502200_papers.md` is created inside the
directory.
