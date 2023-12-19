# Every manifest file consists of the following sections:

    cargo-features — Unstable, nightly-only features.
    [package] — Defines a package.
        name — The name of the package.
        version — The version of the package.
        authors — The authors of the package.
        edition — The Rust edition.
        rust-version — The minimal supported Rust version.
        description — A description of the package.
        documentation — URL of the package documentation.
        readme — Path to the package’s README file.
        homepage — URL of the package homepage.
        repository — URL of the package source repository.
        license — The package license.
        license-file — Path to the text of the license.
        keywords — Keywords for the package.
        categories — Categories of the package.
        workspace — Path to the workspace for the package.
        build — Path to the package build script.
        links — Name of the native library the package links with.
        exclude — Files to exclude when publishing.
        include — Files to include when publishing.
        publish — Can be used to prevent publishing the package.
        metadata — Extra settings for external tools.
        default-run — The default binary to run by cargo run.
        autobins — Disables binary auto discovery.
        autoexamples — Disables example auto discovery.
        autotests — Disables test auto discovery.
        autobenches — Disables bench auto discovery.
        resolver — Sets the dependency resolver to use.
    Target tables: (see configuration for settings)
        [lib] — Library target settings.
        [[bin]] — Binary target settings.
        [[example]] — Example target settings.
        [[test]] — Test target settings.
        [[bench]] — Benchmark target settings.
    Dependency tables:
        [dependencies] — Package library dependencies.
        [dev-dependencies] — Dependencies for examples, tests, and benchmarks.
        [build-dependencies] — Dependencies for build scripts.
        [target] — Platform-specific dependencies.
    [badges] — Badges to display on a registry.
    [features] — Conditional compilation features.
    [lints] — Configure linters for this package.
    [patch] — Override dependencies.
    [replace] — Override dependencies (deprecated).
    [profile] — Compiler settings and optimizations.
    [workspace] — The workspace definition.
