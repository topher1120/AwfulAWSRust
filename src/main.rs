use structopt::StructOpt;
use rusoto_sqs::{GetQueueUrlRequest, SqsClient, Sqs, GetQueueUrlError, GetQueueUrlResult};
use rusoto_core::region::Region;
use rusoto_core::RusotoError;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    #[structopt()]
    queue_name: String
}

#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    println!("Hello, finding SQS URL for queue name: {}", opt.queue_name);

    let request = GetQueueUrlRequest{
        queue_name: opt.queue_name.clone(),
        ..Default::default()
    };
    let sqs_client = SqsClient::new(Region::UsWest2);
    let result = sqs_client.get_queue_url(request).await;
    match result {
        Ok(queue_url_result) => {
            println!("Got a result from the service, queue url is: {}", queue_url_result.queue_url.unwrap());
        }
        Err(err) => {
            eprintln!("Error calling the service: {}", err);
        }
    }
}
