terraform {
    backend "gcs" {
        bucket = "nikitavbv_tf_state"
        prefix = "manticore/kpiexport-api"
    }
}

provider "google" {
    project = "nikitavbv"
    region = "europe-central2"
}

provider "google-beta" {
    project = "nikitavbv"
    region = "europe-central2"
}

variable "service_version" {
    type = string
    description = "Version of image to deploy"
    default = "0.1.66"
}

resource "google_cloud_run_service" "api_service" {
    provider = google-beta

    name = "kpiexport-api"
    location = "europe-central2"

    autogenerate_revision_name = true

    metadata {
        annotations = {
            generated-by = "magic-modules"
            "run.googleapis.com/ingress" = "all"
            "run.googleapis.com/launch-stage" = "BETA"
        }
    }

    template {
        metadata {
            annotations = {
                "autoscaling.knative.dev/maxScale" = "5"
            }
        }

        spec {
            service_account_name = "916750455653-compute@developer.gserviceaccount.com"
    
            containers {
                image = "eu.gcr.io/nikitavbv/nikitavbv/kpiexport:${var.service_version}"
            }
        }
    }
}
