{ ... }:
{
  projectRootFile = "flake.nix";

  programs.rustfmt.enable = true;
  programs.nixfmt.enable = true;
  programs.taplo.enable = true;
}
