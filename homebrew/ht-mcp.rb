class HtMcp < Formula
  desc "Headless Terminal MCP Server - Control terminal sessions via Model Context Protocol"
  homepage "https://github.com/memextech/ht-mcp"
  version "0.1.0"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/memextech/ht-mcp/releases/download/v#{version}/ht-mcp-aarch64-apple-darwin"
      sha256 "YOUR_ARM64_SHA256_HERE"
    else
      url "https://github.com/memextech/ht-mcp/releases/download/v#{version}/ht-mcp-x86_64-apple-darwin"
      sha256 "YOUR_X86_64_SHA256_HERE"
    end
  else
    url "https://github.com/memextech/ht-mcp/releases/download/v#{version}/ht-mcp-x86_64-unknown-linux-gnu"
    sha256 "YOUR_LINUX_SHA256_HERE"
  end

  def install
    bin.install Dir["*"].first => "ht-mcp"
  end

  test do
    # Test that the binary exists and shows version/help
    assert_match version.to_s, shell_output("#{bin}/ht-mcp --version 2>&1")
  end
end