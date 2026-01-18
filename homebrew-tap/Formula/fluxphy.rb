class Fluxphy < Formula
  desc "File transfer tool with deep instrumentation into the physics of data flux"
  homepage "https://github.com/BGSTA9/fluxphy"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/BGSTA9/fluxphy/releases/download/v#{version}/fluxphy-macos-aarch64"
      sha256 "PLACEHOLDER_SHA256_ARM64"
      def install
        bin.install "fluxphy-macos-aarch64" => "fluxphy"
      end
    else
      url "https://github.com/BGSTA9/fluxphy/releases/download/v#{version}/fluxphy-macos-x86_64"
      sha256 "PLACEHOLDER_SHA256_X86_64"
      def install
        bin.install "fluxphy-macos-x86_64" => "fluxphy"
      end
    end
  end

  on_linux do
    url "https://github.com/BGSTA9/fluxphy/releases/download/v#{version}/fluxphy-linux-x86_64"
    sha256 "PLACEHOLDER_SHA256_LINUX"
    def install
      bin.install "fluxphy-linux-x86_64" => "fluxphy"
    end
  end

  test do
    assert_match "fluxphy #{version}", shell_output("#{bin}/fluxphy --version")
  end
end
