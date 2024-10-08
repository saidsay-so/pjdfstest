<!-- omit in toc -->

# Contributing to pjdfstest

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
>
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->

## Table of contents

- [Contributing to pjdfstest](#contributing-to-pjdfstest)
  - [Table of contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [I have a question](#i-have-a-question)
  - [I want to contribute](#i-want-to-contribute)
    - [Reporting bugs](#reporting-bugs)
      - [Before submitting a bug report](#before-submitting-a-bug-report)
      - [How do I submit a good bug report?](#how-do-i-submit-a-good-bug-report)
    - [Suggesting enhancements](#suggesting-enhancements)
      - [Before submitting an enhancement](#before-submitting-an-enhancement)
      - [How Do I submit a good enhancement suggestion?](#how-do-i-submit-a-good-enhancement-suggestion)
    - [Your first code contribution](#your-first-code-contribution)
    - [Improving the documentation](#improving-the-documentation)
  - [Styleguides](#styleguides)
    - [Commit messages](#commit-messages)
  - [Attribution](#attribution)

## Code of Conduct

This project and everyone participating in it is governed by the
[pjdfstest Code of Conduct](https://github.com/saidsay-so/pjdfstestblob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <sayafdine.said@outlook.fr>.

## I have a question

> If you want to ask a question, we assume that you have read the available [Documentation](https://saidsay-so.github.io/pjdfstest/).

Before you ask a question, it is best to search for existing [Issues](https://github.com/saidsay-so/pjdfstest/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [issue](https://github.com/saidsay-so/pjdfstest/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I want to contribute

> ### Legal notice <!-- omit in toc -->
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting bugs

<!-- omit in toc -->

#### Before submitting a bug report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://saidsay-so.github.io/pjdfstest/). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/saidsay-so/pjdfstestissues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->

#### How do I submit a good bug report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead sensitive bugs must be sent by email to <sayafdine.said@outlook.fr>.

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/saidsay-so/pjdfstest/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

<!-- You might want to create an issue template for bugs and errors that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Suggesting enhancements

This section guides you through submitting an enhancement suggestion for pjdfstest, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->

#### Before submitting an enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://saidsay-so.github.io/pjdfstest/) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/saidsay-so/pjdfstest/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->

#### How Do I submit a good enhancement suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/saidsay-so/pjdfstest/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- **Explain why this enhancement would be useful** to most pjdfstest users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

<!-- You might want to create an issue template for enhancement suggestions that can be used as a guide and that defines the structure of the information to be included. If you do so, reference it here in the description. -->

### Your first code contribution

The project team welcomes your contributions.
Before you start, we want to make sure that you have read the [documentation](https://saidsay-so.github.io/pjdfstest/)
and that you have followed the steps in the [Reporting Bugs](#reporting-bugs) section.

To get started, you need to set up your development environment. You can find instructions on how to do this in the [README](README.md).

### Improving the documentation

The documentation is a crucial part of the project. It is the first place users will look when they want to understand a feature, and the first place they will go when they are looking for help. Therefore, it is important that the documentation is up-to-date, accurate, and easy to understand.

The book is written in Markdown and is located in the `book` directory. It is built using mdbook.
You can find instructions on how to install it in the [mdbook documentation](https://rust-lang.github.io/mdBook/cli/index.html).

You can preview the book by running the following command:

```bash
cd book
mdbook serve
```

To build the book, run the following command:

```bash
cd book
./build.sh
```

This will generate the book in the `build` directory and add the crate documentation alongside it.

## Styleguides

### Commit messages

Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This leads to more readable messages that are easy to follow when looking through the project history. The commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

The `type` must be one of the following:

- `build`: Changes that affect the build system or external dependencies (cargo metadata)
- `ci`: Changes to our CI configuration files and scripts (GitHub Actions, Cirrus CI)
- `docs`: Documentation only changes
- `feat`: A new feature
- `fix`: A bug fix
- `perf`: A code change that improves performance
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `style`: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools and libraries such as documentation generation
- `revert`: Reverts a previous commit
- `release`: Changes to the release process

The `scope` is optional and can be anything specifying the place of the commit change.

The `description` should be a short, concise description of the change.

The `body` is optional and can provide more detailed information about the change.

The `footer` is optional and can be used to reference issues or pull requests.

Here are some examples of commit messages:

```conventionalcommit
feat(parser): add support for arrays
```

```conventionalcommit
fix(parser): handle empty values
```

## Attribution

This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
