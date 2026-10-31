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
    hash = "sha256-N2rRqqoehHbxFWlucOZBxy4RnkT2LUdrsky9DOsflHc==";
  };

  cargoHash = "sha256-BNXHycv1iIOUPrGjv9HlPwJCaQncKnKegUiUoQ7MMcw=";

  meta = {
    description = "Render certain fractals in a terminal";
    homepage = "https://github.com/Definitely-Not-A-Dolphin/Fractal-Renderer";
    license = lib.licenses.mit;
    mainProgram = "fractal-renderer";
    maintainers = [ lib.maintainers.killioiden ];
  };
})
