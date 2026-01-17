class Fluxphy < Formula
  desc "File transfer tool with deep instrumentation into the physics of data flux"
  homepage "https://github.com/BGSTA9/fluxphy"
  url "https://github.com/BGSTA9/fluxphy/archive/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/fluxphy", "--version"
  end
end
