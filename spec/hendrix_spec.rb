require "socket"

RSpec.describe "Hexdrix" do
  let(:server) { TCPSocket.new("localhost", 9827) }

  it "echoes a message" do
    server.puts("foo")
    message = server.gets

    expect(message).to eq("foo\n")
  end
end
