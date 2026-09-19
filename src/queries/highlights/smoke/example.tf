# café: Terraform and HCL, never executed
variable "region" {
  type    = string
  default = "earth"
}
locals {
  message = "hello ${upper(var.region)}"
  literal = "literal $${not_an_expression} and \"quotes\""
  enabled = true
  absent  = null
  count   = 42
  numbers = [for n in [1, 2] : n * 2 if n > 0]
  mapping = { for k, v in { a = 1 } : k => v }
  choice  = true ? "yes" : "no"
  ids     = var.items[*].id
  raw     = <<EOT
raw café ${lower(var.region)}
EOT
  text = <<-END
    %{ if true ~}
    %{ for item in var.items ~}
    item ${item}
    %{ endfor ~}
    %{ else ~}
    empty
    %{ endif ~}
  END
  // line comment
  /* block comment */
  after = "still highlighted"
}
