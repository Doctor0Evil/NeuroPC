terraform {
  required_version = ">= 1.0.7"

  required_providers {
    tfe = {
      source  = "hashicorp/tfe"
      version = ">= 0.38.0"
    }
    http = {
      source  = "hashicorp/http"
      version = ">= 3.4.0"
    }
  }
}

variable "tfc_org" {
  type        = string
  description = "HCP Terraform org name hosting the workspaces."
}

variable "workspace_prefix" {
  type        = string
  description = "Workspace name prefix allowed to call this neuromorphic run task."
}

variable "neuropc_endpoint" {
  type        = string
  description = "Base URL of the neuromorphic-terraformer API (your NeuroPC gateway)."
}

variable "neuropc_api_token" {
  type        = string
  sensitive   = true
  description = "Token used by terraform run-task to authenticate to your NeuroPC endpoint."
}

resource "tfe_run_task" "neuropc_terraformer" {
  name         = "neuromorphic-terraformer-neuropc"
  url          = "${var.neuropc_endpoint}/api/v1/terraform/run-task"
  description  = "NeuroPC sovereign neuromorphic-terraformer (RoH<=0.3, neurorights aware)."
  enabled      = true
  enforcement_level = "advisory" # can be 'mandatory' after validation

  workspace_external_ids = [
    // pattern: only workspaces with this prefix are allowed
    // AI-Chat or human can expand this list.
  ]
}

output "neuropc_runtask_id" {
  description = "RunTask id for neuromorphic-terraformer."
  value       = tfe_run_task.neuropc_terraformer.id
}
