pub mod wake;

#[get("/")]
pub fn index() -> &'static str {
	"ProjectOS localAPI"
}

