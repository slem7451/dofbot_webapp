import json

def test(*args):
    data = {'test': args}
    with open('result.json', 'w') as file:
        json.dump(data, file)