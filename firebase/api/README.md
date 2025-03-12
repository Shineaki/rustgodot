# How to use

* Install gcloud
* Run gcloud init
* gcloud config set run/region europe-west6
* Run gcloud run deploy api --port 8080 --source .
* Done

uvicorn app.main:app --host 0.0.0.0 --port 8080 --reload