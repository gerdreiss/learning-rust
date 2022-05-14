import concurrent.futures
import os
import requests
import threading

from bs4 import BeautifulSoup

'''
URL of the archive web-page which provides link to
all ngram archives. It would have been tiring to
download each archive manually.
In this example, we first crawl the webpage to extract
all the links and then download archives.
'''

# specify the URL of the archive here
archive_url = "http://storage.googleapis.com/books/ngrams/books/20200217/eng/eng-1-ngrams_exports.html"

thread_local = threading.local()


def get_session():
    if not hasattr(thread_local, "session"):
        thread_local.session = requests.Session()
    return thread_local.session


def get_archive_links():

    # create response object
    r = requests.get(archive_url)

    # create beautiful-soup object
    soup = BeautifulSoup(r.content, 'html5lib')

    # find all links on web-page
    links = soup.findAll('a')

    # filter the link sending with .gz
    return [link['href'] for link in links if link['href'].endswith('gz')]


def download_archive(archive_link):
    # obtain filename by splitting url and getting
    # last string
    file_name = archive_link.split('/')[-1]

    print("Downloading file: %s" % file_name)

    with get_session().get(archive_link, stream=True) as r:
        # download started
        with open(file_name, 'wb') as f:
            for chunk in r.iter_content(chunk_size=1024*1024):
                if chunk:
                    f.write(chunk)

        print("%s downloaded!\n" % file_name)


def download_archives_par(archive_links):
    '''iterate through all links in archive_links
    and download them in parallel'''
    with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count()) as executor:
        executor.map(download_archive, archive_links)


def download_archives(archive_links):
    '''iterate through all links in archive_links
    and download them one by one'''

    for link in archive_links:

        # obtain filename by splitting url and getting
        # last string
        file_name = link.split('/')[-1]

        print("Downloading file: %s" % file_name)

        # create response object
        r = requests.get(link, stream=True)

        # download started
        with open(file_name, 'wb') as f:
            for chunk in r.iter_content(chunk_size=1024*1024):
                if chunk:
                    f.write(chunk)

        print("%s downloaded!\n" % file_name)

    print("All archives downloaded!")
    return


if __name__ == "__main__":

    # getting all archive links
    archive_links = get_archive_links()

    # download all archives
    download_archives_par(archive_links)

    print("All archives downloaded!")
