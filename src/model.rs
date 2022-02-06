//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token{
    pub AccessToken: String,
    Expires: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub greeting: String
}
