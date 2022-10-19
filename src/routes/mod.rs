pub mod wake;
pub mod ping;

#[get("/")]
pub fn index() -> &'static str {
	"ProjectOS localAPI"
}

