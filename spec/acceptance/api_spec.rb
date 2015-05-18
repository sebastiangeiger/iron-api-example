require_relative '../../spec/spec_helper'

describe 'API', type: :feature do
  describe 'GET /' do
    it 'gets hello world' do
      visit('/')
      expect(page.status_code).to eql 200
      expect(page).to have_content "Hello World"
    end
  end

  describe 'GET /someotherroute' do
    it 'returns a 404' do
      begin
        visit('/someotherroute')
        fail "Should not reach this"
      rescue Object => e
        expect(e.message).to include "404 => Net::HTTPNotFound"
      end
    end
  end
end


