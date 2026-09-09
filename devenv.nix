{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.cmake
    pkgs.cmake-language-server
  ];

  # https://devenv.sh/languages/
  languages.cplusplus = {
    enable = true;
    lsp.package = pkgs.clang;
  };

  # https://devenv.sh/scripts/
  scripts.build = {
    exec = ''
      set -euo pipefail
      cmake -S "$DEVENV_ROOT" -B "$DEVENV_ROOT/build" -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
      if [ $# -lt 1 ]; then
        cmake --build "$DEVENV_ROOT/build"
        exit 0
      fi
      project="$1"
      src="$DEVENV_ROOT/$project"
      if [ ! -f "$src/CMakeLists.txt" ]; then
        echo "Unknown project: $project" >&2
        exit 1
      fi
      cmake --build "$DEVENV_ROOT/build" --target "$project"
      exe=$(sed -n 's/.*add_executable([[:space:]]*\([^[:space:])]*\).*/\1/p' "$src/CMakeLists.txt" | head -n 1)
      if [ -n "$exe" ]; then
        cmake --build "$DEVENV_ROOT/build" --target "$exe"
      fi
    '';
    description = "Build all projects, or one by name";
  };

  scripts.run = {
    exec = ''
      set -euo pipefail
      if [ $# -lt 1 ]; then
        echo "Usage: run <project> [args...]" >&2
        exit 1
      fi
      project="$1"
      shift
      src="$DEVENV_ROOT/$project"
      if [ ! -f "$src/CMakeLists.txt" ]; then
        echo "Unknown project: $project" >&2
        exit 1
      fi
      exe=$(sed -n 's/.*add_executable([[:space:]]*\([^[:space:])]*\).*/\1/p' "$src/CMakeLists.txt" | head -n 1)
      if [ -z "$exe" ]; then
        echo "Project '$project' has no executable" >&2
        exit 1
      fi
      cmake -S "$DEVENV_ROOT" -B "$DEVENV_ROOT/build" -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
      cmake --build "$DEVENV_ROOT/build" --target "$exe"
      exec "$DEVENV_ROOT/build/$project/$exe" "$@"
    '';
    description = "Run a project's executable";
  };

  # https://devenv.sh/git-hooks/
  git-hooks.hooks = {
    clang-format.enable = true;
    clang-tidy = {
      enable = true;
      args = [
        "-p"
        "build"
        "--config-file=.clang-tidy"
      ];
    };
    cmake-format.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
