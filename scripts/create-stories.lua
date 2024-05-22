local counter = 0

request = function()
    counter = counter + 1
    headers = {}
    headers["Content-Type"] = "application/json"
    body = '{"name": "Story ' .. counter .. '"}'
    return wrk.format("POST", "/stories", headers, body)
end
