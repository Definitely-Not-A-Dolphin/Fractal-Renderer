{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "fractal-renderer";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "Definitely-Not-A-Dolphin";
    repo = "Fractal-Renderer";
    tag = finalAttrs.version;
    hash = "sha256-VYNDfirVln7zLNqQBThPDBzPLcX414XcdBQ0iOFziO0=";
  };

  cargoHash = "sha256-UQJ3gKYyXHS3IgaLHAqF/PM2Hy/8g7vJxfb7bFZsJBA=";

  meta = {
    description = "Render certain fractals in a terminal";
    homepage = "https://github.com/Definitely-Not-A-Dolphin/Fractal-Renderer";
    license = lib.licenses.mit;
    mainProgram = "fractal-renderer";
    maintainers = [ lib.maintainers.killioiden ];
  };
})
