# How to use

* Install gcloud
* Run gcloud init
* gcloud config set run/region europe-west6
* Run gcloud run deploy api --port 8080 --source .
* Done

# Start API locally

First you need to get a service account json config from Firebase, then you can run:

uvicorn app.main:app --host 0.0.0.0 --port 8080 --reload