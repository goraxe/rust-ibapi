use ibapi::{orders::OrderDataResult, Client};

fn main() {
    let client_url = std::env::var("CLIENT_URL").expect("CLIENT_URL must be set");
    let account_id = std::env::var("ACCOUNT_ID").expect("ACCOUNT_ID must be set");
    let client = Client::connect(&client_url, 919).expect("connection failed");

    // Positions
    let positions = client.positions().expect("request failed");
    for position in positions {
        println!("{:4} {:4} @ {}", position.position, position.contract.symbol, position.average_cost)
    }

    fn print_orders(orders: impl Iterator<Item = OrderDataResult>) {
        for order in orders {
            println!("order: {order:?}")
        }
    }

    // Open orders
    let open_orders = client.all_open_orders().expect("request failed");
    print_orders(open_orders);

    // PnL
    let pnl = client.pnl(&account_id).expect("request failed");
    println!("PnL: {:?}", pnl);

    // Account summary
    let account_summary = client.account_summary("All", "NetLiquidation").expect("request failed");
    println!("Account Summary: {:?}", account_summary);
}
