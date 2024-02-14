import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type ViteDevServer } from 'vite';
import { Server, Socket } from 'socket.io';

const webSocketServer = {
	name: 'webSocketServer',
	configureServer(server: ViteDevServer) {
		if (!server.httpServer) return;

		let listSocketsConnected: Socket[] = [];

		const io = new Server(server.httpServer);

		io.on('connection', (socket) => {
			listSocketsConnected.push(socket);
			console.log(socket.handshake.query);
			socket.emit('eventFromServer', 'connected 👋');
			socket.on('accept-bet', (value) => {
				const socketMessage = listSocketsConnected.find(
					(socket) => String(socket.handshake.query.uid) === String(value.uid)
				);
				socketMessage?.emit('confirm-bet', value);
			});
			socket.on('bet-placed', (value) => {
				io.emit('bet-placed', value);
			});
			socket.on('disconnect', () => {
				listSocketsConnected = [...listSocketsConnected.filter((sck) => sck !== socket)];
			});
		});
	}
};

export default defineConfig({
	plugins: [sveltekit(), purgeCss(), webSocketServer]
});
