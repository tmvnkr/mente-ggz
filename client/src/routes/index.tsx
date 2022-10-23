import { component$, useResource$, Resource } from "@builder.io/qwik";

export default component$(() => {
  const testDataResource = useResource$<string[]>(({ cleanup }) => {
    const controller = new AbortController();
    cleanup(() => controller.abort());

    return getTestData(controller);
  });

  return (
    <div>
      <h1>
        Welcome to Qwik <span class="lightning">⚡️</span>
      </h1>
      <Resource
        value={testDataResource}
        onResolved={(data) => (
          <ul>
            {data.map((data) => (
              <li>{data}</li>
            ))}
          </ul>
        )}
      />
    </div>
  );
});

export async function getTestData(
  controller?: AbortController
): Promise<string[]> {
  const resp = await fetch("http://localhost:4000/test", {
    signal: controller?.signal,
  });

  const json = await resp.json();

  return Array.isArray(json) ? json : Promise.reject(json);
}
