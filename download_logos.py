import requests
from bs4 import BeautifulSoup
import os

# Create a directory to save logos
if not os.path.exists('logos'):
    os.makedirs('logos')

# URL of the website
url = 'https://www.feylogos.com/'

# Send a request to fetch the HTML content
response = requests.get(url)
soup = BeautifulSoup(response.text, 'html.parser')

# Find all logo elements (update the tag and class based on the website's HTML structure)
logo_elements = soup.find_all('div', class_='sc-bc3d610e-52')

# Download each logo
for logo in logo_elements:
    div_id = logo['id']
    img_tag = logo.find('img')
    img_url = img_tag['src']

    # Download the image
    img_response = requests.get(img_url)
    img_filename = f'logos/{div_id}.svg'
    with open(img_filename, 'wb') as file:
        file.write(img_response.content)
        print(f'Downloaded {img_filename}')

print('All logos downloaded successfully!')

