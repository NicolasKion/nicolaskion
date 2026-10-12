<script lang="ts">
  import ProjectsPage from '#lib/components/projects-page.svelte';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<ProjectsPage props={data} />
