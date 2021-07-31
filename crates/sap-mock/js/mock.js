export function mock_fetch_resolve(value) {
	let original_fetch = fetch;
	fetch = () => {
		if (typeof value === "object") {
			value = new Blob([JSON.stringify(value, null, 2)]);
		}
		return new Promise((resolve) => {
			resolve(new Response(value, {
				status: 200,
			}))
		});
	};
	return original_fetch;
}

export function mock_fetch_error(code, reason) {
	function reject_mock() {
		return new Promise((resolve, reject) => {
			reject(reason);
		})
	};
	let original_fetch = fetch;
	fetch = () => {
		const blob = new Blob([JSON.stringify(reason, null, 2)]);
		return new Promise((resolve) => {
			let resp = new Response(blob, { status: code });
			resp.arrayBuffer = reject_mock;
			resp.blob = reject_mock;
			resp.formData = reject_mock;
			resp.json = reject_mock;
			resp.text = reject_mock;
			resolve(
				resp
			)
		})
	};
	return original_fetch;
}


export function restore_fetch(original_fetch) {
	fetch = original_fetch;
}

export function mock_websocket(conn_delay) {
	let mock_controller = {
		is_opened: false,
		last_message: ``,
		last_message_type: ``,
		original_ws: WebSocket,
		send: undefined,
		close: undefined,
		error: undefined,
		restore: () => {
			if (mock_controller.close) {
				// 1001 - Going away as object being dropped in Rust.
				mock_controller.close(1001);
			}
			WebSocket = mock_controller.original_ws;
		}
	};

	WebSocket = class MockWebSocket extends EventTarget {
		static CONNECTING = 0;
		static OPEN = 1;
		static CLOSING = 2;
		static CLOSED = 3;

		constructor(url, protocols) {
			super();
			this.url = url;
			this.protocols = protocols;
			this.readyState = MockWebSocket.CONNECTING;
			let wait = setTimeout(() => {
				clearTimeout(wait);
				this.readyState = MockWebSocket.OPEN;
				const event = new Event(`open`);
				this.dispatchEvent(event);
				this.onopen(event);
				mock_controller.is_opened = true;
			}, conn_delay);

			mock_controller.send = (data) => {
				const event = new MessageEvent(`message`, {
					data: data,
					origin: `WebSocket`,
				});
				this.dispatchEvent(event);
				this.onmessage(event);
			}

			mock_controller.error = (message) => {
				const event = new ErrorEvent(`error`, {
					message: message
				});
				this.dispatchEvent(event);
				this.onerror(event);
			}

			mock_controller.close = (code, reason) => {
				this.close(code, reason);
				// avoid closing twice
				mock_controller.close = undefined;
			}
		}

		onopen(e) { }
		onerror(e) { }
		onclose(e) { }
		onmessage(e) { }

		send(data) {
			mock_controller.last_message_type = typeof data;
			mock_controller.last_message = data;
		}

		close(code, reason) {
			this.readyState = MockWebSocket.CLOSED;
			const event = new CloseEvent(`close`, {
				code: code,
				reason: reason,
			});
			this.dispatchEvent(event);
			this.onclose(event);
		}
	}

	globalThis.mock = mock_controller;

	return mock_controller;
}