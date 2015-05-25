require_relative '../../spec/spec_helper'
require "sqlite3"

def connection
  @connection ||= Faraday.new(:url => 'http://localhost:3000')
end

def get(path, options = {})
  connection.get(path) do |request|
    request.headers['Content-Type'] = 'application/json'
    request.headers['X-Auth-Token'] = options[:auth_token] if options[:auth_token]
  end
end

def post(path, options = {})
  connection.post(path) do |request|
    request.body = JSON.dump(options[:json]) if options[:json]
    request.headers['Content-Type'] = 'application/json'
    request.headers['X-Auth-Token'] = options[:auth_token] if options[:auth_token]
  end
end

describe 'API', type: :feature do
  let(:json) { JSON.parse(response.body) }

  after(:each) do
    db = SQLite3::Database.new "test.sqlite3"
    db.execute "DELETE FROM items"
  end

  let(:auth_token) { "VALID_TOKEN" }

  describe 'GET /items' do
    subject(:response) { get('/items', auth_token: auth_token) }

    context 'when authorized' do
      it 'gets a 200 response' do
        expect(response.status).to eql 200
      end

      it 'gets an empty list' do
        expect(json).to eql []
      end

      context 'after adding an item' do
        before { post('/items', json: { name: "Apples" }, auth_token: auth_token) }

        it 'gets a list with the added item' do
          expect(json).to eql [{"name" => "Apples"}]
        end
      end
    end

    context 'when not authorized' do
      let(:auth_token) { nil }

      it 'gets a 401 response' do
        expect(response.status).to eql 401
      end

      it 'tells you to supply a X-Auth-Token' do
        expect(json).to eql({ "error" => "Please supply a X-Auth-Token header" })
      end
    end
  end

  describe 'GET /someotherroute' do
    it 'gets a 404 response' do
      expect(get('/someotherroute', auth_token: auth_token).status).to eql 404
    end
  end

  describe 'POST /items' do
    subject(:response) { post('/items', json: payload, auth_token: auth_token) }

    context 'with a valid payload' do
      let(:payload) { {name: "Bananas"} }

      it 'gets a 200 response' do
        expect(response.status).to eql 200
      end

      it 'mirrors back the item passed in' do
        expect(json).to eql({"name" => "Bananas"})
      end
    end

    context 'with an invalid payload' do
      let(:payload) { {something: "else"} }

      it 'gets a 422 response' do
        expect(response.status).to eql 422
      end

      it 'has an error message' do
        expect(json).to eql({"error" => "Invalid payload"})
      end
    end
  end
end


