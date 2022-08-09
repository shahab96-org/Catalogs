resource "aws_lambda_function" "this" {
  function_name = local.project_prefix
  filename      = "../../dist/lambda.zip"
  description   = filesha256("../../dist/lambda.zip")
  handler       = "bootstrap"
  runtime       = "provided.al2"
  memory_size   = 128
  architectures = ["x86_64"]
  timeout       = 900
  role          = aws_iam_role.this.arn

  environment {
    variables = {
      STAGE      = terraform.workspace
    }
  }
}

resource "aws_lambda_permission" "this" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.this.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.this.execution_arn}/*/*/*"
}

resource "aws_cloudwatch_log_group" "this" {
  name              = "/aws/lambda/${local.project_prefix}"
  retention_in_days = 30
}
