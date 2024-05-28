import urllib.request
import json

def get_random_joke():
    url = "https://v2.jokeapi.dev/joke/Any"
    try:
        with urllib.request.urlopen(url) as response:
            if response.status == 200:
                data = json.loads(response.read().decode())
                if data['type'] == 'single':
                    # Single-part joke
                    print(f"Joke: {data['joke']}")
                elif data['type'] == 'twopart':
                    # Two-part joke (setup and delivery)
                    print(f"Setup: {data['setup']}")
                    print(f"Delivery: {data['delivery']}")
            else:
                print(f"Error fetching joke: {response.status}")
    except urllib.error.URLError as e:
        print(f"Failed to fetch joke: {e.reason}")

if __name__ == "__main__":
    get_random_joke()