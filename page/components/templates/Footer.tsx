import { Typography } from "@mui/material";

export default function Footer() {
	const year = new Date().getFullYear();
	return (
		<footer>
			<Typography sx={{ textAlign: "center" }}>&copy; {year} Takumi Mori</Typography>
		</footer>
	);
}
