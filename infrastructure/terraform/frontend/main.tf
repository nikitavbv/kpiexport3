terraform {
    backend "gcs" {
        bucket = "nikitavbv_tf_state"
        prefix = "manticore/kpiexport-frontend"
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
    default = "0.1.76"
}

resource "google_cloud_run_service" "kpiexport_service" {
    provider = google-beta

    name = "kpiexport"
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
                "run.googleapis.com/vpc-access-connector" = "projects/nikitavbv/locations/europe-central2/connectors/cloud-run-api-connector-w"
                "run.googleapis.com/vpc-access-egress"    = "all-traffic"
            }
        }

        spec {
            service_account_name = "916750455653-compute@developer.gserviceaccount.com"
    
            containers {
                image = "eu.gcr.io/nikitavbv/nikitavbv/kpiexport_frontend:${var.service_version}"
            }
        }
    }
}
