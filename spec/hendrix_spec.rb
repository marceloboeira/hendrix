require "socket"

RSpec.describe "Hexdrix" do
  let(:peer_a) { TCPSocket.new("localhost", 9827) }
  let(:peer_b) { TCPSocket.new("localhost", 9827) }

  it "echoes a message" do
    peer_a.puts("foo")
    message = peer_a.gets

    expect(message).to eq("foo\n")
  end

  it "echoes a message to the correct connection only" do
    peer_a.puts("foo")
    peer_b.puts("bar")
    message = peer_b.gets

    expect(message).to eq("bar\n")
  end
end
