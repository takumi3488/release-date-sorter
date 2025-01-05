import { Box, CssBaseline } from "@mui/material";
import Footer from "./templates/Footer";
import Header from "./templates/Header";

export default function Layout({ children }: { children: React.ReactNode }) {
	return (
		<>
			<CssBaseline />
			<Box
				component="div"
				sx={{
					display: "flex",
					flexDirection: "column",
					minHeight: "100dvh",
					gap: "1rem",
					p: 2,
				}}
			>
				<Header />
				<Box component="main" sx={{ flexGrow: 1 }}>
					{children}
				</Box>
				<Footer />
			</Box>
		</>
	);
}
