import { Divider, Typography } from "@mui/material";
import { Link } from "wouter";

export default function Header() {
	return (
		<header>
			<Link href="/" style={{ textDecoration: "none", color: "indigo" }}>
				<Typography component="h1" variant="h3" fontFamily="Yuji Syuku">
					発売日順に並び替えるやつ
				</Typography>
			</Link>
			<Divider sx={{ pt: "1rem" }} />
		</header>
	);
}
