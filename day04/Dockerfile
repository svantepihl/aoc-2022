FROM mcr.microsoft.com/dotnet/sdk:7.0-alpine AS build

WORKDIR /src
COPY . .
RUN dotnet restore
RUN dotnet publish -c Release -o /app -r linux-musl-x64 --no-restore

FROM mcr.microsoft.com/dotnet/runtime:7.0-alpine

WORKDIR /app
COPY --from=build /app/aoc .
COPY input*.txt ./

ENTRYPOINT ["./aoc"]