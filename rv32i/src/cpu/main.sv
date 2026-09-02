module main;
  logic clk;
  logic rst;

  string exec_file;
  string trace_file;
  string log_file;
  string vcd_file;

  int trace_fd;
  int log_fd;

  logic [31:0] mem[0:255];
  int bin_file;
  logic [31:0] addr, data;
  logic [31:0] mem_index, data_raw;

  initial begin
    clk = 1'b0;
    rst = 1'b0;
    forever #5 clk = ~clk;
  end

  initial begin
    if (!$value$plusargs("EXEC=%s", exec_file)) exec_file = "program.bin";
    if (!$value$plusargs("TRACE_FILE=%s", trace_file)) trace_file = "trace.log";
    if (!$value$plusargs("LOG_FILE=%s", log_file)) log_file = "stdout.log";
    if (!$value$plusargs("VCD_FILE=%s", vcd_file)) vcd_file = "rv32i.vcd";

    bin_file = $fopen(exec_file, "rb");
    if (bin_file == 0) begin
      $display(log_fd, "error: cannot open file: %s", exec_file);
    end else begin
      int read_bytes;
      read_bytes = $fread(mem, bin_file);
      $fclose(bin_file);
      $display("loaded %0d bytes", read_bytes);
    end

    trace_fd = $fopen(trace_file, "w");
    log_fd   = $fopen(log_file, "w");

    $dumpfile(vcd_file);
    $dumpvars(0, rv32i_);

    rst = 1'b1;
    #20;
    rst = 1'b0;

    #1000;
    $fclose(trace_fd);
    $fclose(log_fd);

    $finish;
  end

  always_comb begin
    mem_index = (addr - 32'h80000000) >> 2;
    data_raw = mem[mem_index];
    data = (data_raw >> 24) | 
           ((data_raw >> 8) & 32'h0000FF00) | 
           ((data_raw << 8) & 32'h00FF0000) | 
           (data_raw << 24);
  end

  rv32i rv32i_ (
      .clk     (clk),
      .rst     (rst),
      .addr    (addr),
      .data    (data),
      .trace_fd(trace_fd),
      .log_fd  (log_fd)
  );

endmodule
