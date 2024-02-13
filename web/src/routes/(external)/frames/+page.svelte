<script>
	import { page } from '$app/stores';
</script>

<div
	class="h-full w-full bg-cover bg-center bg-no-repeat"
	style={`background-image: url('${$page.url.searchParams.get('src')}');`}
/>
