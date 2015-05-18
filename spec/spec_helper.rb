require 'capybara/rspec'
require 'capybara/mechanize'

Capybara.register_driver :mechanize do |app|
  Capybara::Mechanize::Driver.new(app)
end

Capybara.configure do |config|
  config.run_server = false
  config.default_driver = :mechanize
  config.app_host = "http://localhost:3000"
  config.app = true # This is required :-/
end

RSpec.configure do |config|
  config.before(:all) do
    raise "`cargo build` failed" unless system("cargo build")
    @server_pid = fork do
      exec "target/debug/iron-api-example"
    end
  end

  config.after(:all) do
    Process.kill("INT", @server_pid)
    Process.wait
  end
end

RSpec::Matchers.define :have_status_code do |expected|
  match do |actual|
      if actual.respond_to?(:call)
        begin
          actual.call
          page.status_code == expected
        # TODO: Swallowing errors here, fine for now
        rescue Object => e
          case expected
          when 404
            e.message.include? "404 => Net::HTTPNotFound"
          else
            fail "Please teach matcher how to handle status_code #{expected}"
          end
        end
      else
        actual.status_code == expected
      end
  end

  def supports_block_expectations?
    true
  end
end
