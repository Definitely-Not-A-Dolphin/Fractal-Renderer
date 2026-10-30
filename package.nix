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
    hash = "sha256-xAjn0fGdamcH82scAemXJYkkfr6ew11HqGAA7mM1N6k=";
  };

  cargoHash = "sha256-gxnxcMWXcaueL+gSbjIaYbomJ70Znatlwq4ER5+5zoo=";

  meta = {
    description = "Render certain fractals in a terminal";
    homepage = "https://github.com/Definitely-Not-A-Dolphin/Fractal-Renderer";
    license = lib.licenses.mit;
    mainProgram = "fractal-renderer";
    maintainers = [ lib.maintainers.killioiden ];
  };
})
