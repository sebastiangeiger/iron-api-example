require_relative '../../spec/spec_helper'

def connection
  @connection ||= Faraday.new(:url => 'http://localhost:3000')
end

def get(path)
  connection.get(path)
end

def post(path, options = {})
  connection.post(path) do |request|
    request.body = JSON.dump(options[:json]) if options[:json]
  end
end

describe 'API', type: :feature do
  let(:json) { JSON.parse(response.body) }

  describe 'GET /items' do
    subject(:response) { get('/items') }

    it 'gets a 200 response' do
      expect(response.status).to eql 200
    end

    it 'gets an empty list' do
      expect(json).to eql [{"name" => "Bananas"}]
    end
  end

  describe 'GET /someotherroute' do
    it 'gets a 404 response' do
      expect(get('/someotherroute').status).to eql 404
    end
  end

  describe 'POST /items' do
    subject(:response) { post('/items', json: payload) }

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


